use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures::StreamExt;
use reqwest::Client;
use serde_json::Value;
use tower_http::cors::{Any, CorsLayer};
use tracing::{debug, error, info};

use crate::admin_ui;
use crate::config::{self, Config};
use crate::convert::{self, MessagesRequest, ResponsesRequest};

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub config: Arc<RwLock<Config>>,
    pub config_path: RwLock<PathBuf>,
    pub http_client: Client,
}

impl AppState {
    pub fn new(config: Arc<RwLock<Config>>, config_path: PathBuf) -> Self {
        Self {
            config,
            config_path: RwLock::new(config_path),
            http_client: Client::builder()
                .timeout(std::time::Duration::from_secs(300))
                .build()
                .unwrap(),
        }
    }
}

pub fn build_router(state: SharedState) -> Router {
    Router::new()
        .route("/v1/messages", post(handle_messages))
        .route("/v1/messages/count_tokens", post(handle_count_tokens))
        .route("/v1/responses", post(handle_responses))
        .route("/v1/models", get(handle_models))
        .route("/v1/models/{_name}", get(handle_model_detail))
        .route("/health", get(handle_health))
        .route("/", get(handle_health))
        .route("/admin", get(handle_admin_ui))
        .route("/admin/api/config", get(handle_admin_get_config))
        .route("/admin/api/config", post(handle_admin_update_config))
        .route("/admin/api/provider", post(handle_admin_create_provider))
        .route("/admin/api/provider/{id}", axum::routing::delete(handle_admin_delete_provider))
        .route("/admin/api/test", post(handle_admin_test))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(state)
}

// ==================== Anthropic Messages Endpoint ====================

async fn handle_messages(
    State(state): State<SharedState>,
    _headers: axum::http::HeaderMap,
    body: String,
) -> Response {
    let req: MessagesRequest = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(anthropic_error("invalid_request_error", &e.to_string())),
            )
                .into_response();
        }
    };

    // Clone everything we need before spawning the stream
    let (provider_id, provider_model, base_url, api_key, client);
    {
        let config = state.config.read().unwrap();
        let (pid, pm, _original) = config::resolve_model(&config, &req.model);

        let provider = match config.providers.get(pid) {
            Some(p) => p.clone(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(anthropic_error(
                        "invalid_request_error",
                        &format!("未知 provider: {pid}"),
                    )),
                )
                    .into_response();
            }
        };

        if provider.api_key.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(anthropic_error(
                    "invalid_request_error",
                    &format!("Provider {pid} 未配置 API Key，请在 Admin UI 中配置"),
                )),
            )
                .into_response();
        }

        provider_id = pid.to_string();
        provider_model = pm.to_string();
        base_url = provider.base_url().trim_end_matches('/').to_string();
        api_key = provider.api_key.clone();
    }
    client = state.http_client.clone();

    let chat_req = convert::anthropic_to_openai(&req, &provider_model);
    let chat_url = format!("{base_url}/chat/completions");

    debug!(
        "转发请求: {} -> {}/{} (via {})",
        req.model, provider_id, provider_model, chat_url
    );

    let stream_result = client
        .post(&chat_url)
        .header("Authorization", format!("Bearer {api_key}"))
        .header("Content-Type", "application/json")
        .json(&chat_req)
        .send()
        .await;

    let response = match stream_result {
        Ok(r) => r,
        Err(e) => {
            error!("上游请求失败: {e}");
            return (
                StatusCode::BAD_GATEWAY,
                Json(anthropic_error("api_error", &format!("上游服务连接失败: {e}"))),
            )
                .into_response();
        }
    };

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        error!("上游返回错误 {}: {}", status.as_u16(), error_body);
        return (
            StatusCode::BAD_GATEWAY,
            Json(anthropic_error(
                "api_error",
                &format!("上游返回 {status}: {error_body}"),
            )),
        )
            .into_response();
    }

    let mut stream = response.bytes_stream();
    let model_name = provider_model;

    let body_stream = async_stream::stream! {
        let mut converter = convert::SseConverter::new(&model_name);

        if let Some(start) = converter.ensure_message_start() {
            yield Ok::<_, std::convert::Infallible>(start);
        }

        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    buffer.push_str(&text);

                    while let Some(line_end) = buffer.find('\n') {
                        let line = buffer[..line_end].trim().to_string();
                        buffer = buffer[line_end + 1..].to_string();

                        if line.is_empty() || line.starts_with(':') || line == "data: [DONE]" {
                            continue;
                        }

                        if let Some(data) = line.strip_prefix("data: ") {
                            if let Ok(chunk) = serde_json::from_str::<convert::OpenAISSEChunk>(data) {
                                if let Some(usage) = &chunk.usage {
                                    converter.update_usage(usage);
                                }
                                if let Some(choices) = &chunk.choices {
                                    for choice in choices {
                                        let events = converter.process_choice(choice);
                                        for event in events {
                                            yield Ok(event);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("SSE 读取错误: {e}");
                    break;
                }
            }
        }
    };

    let stream = tokio_stream::StreamExt::map(body_stream, |item| match item {
        Ok(s) => Ok::<_, std::convert::Infallible>(s),
    });

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .body(Body::from_stream(stream))
        .unwrap()
        .into_response()
}

// ==================== OpenAI Responses Endpoint ====================

async fn handle_responses(
    State(state): State<SharedState>,
    body: String,
) -> Response {
    let req: ResponsesRequest = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": {"message": format!("请求解析失败: {e}"), "type": "invalid_request_error"}
                })),
            )
                .into_response();
        }
    };

    let (provider_model, base_url, api_key, client);
    {
        let config = state.config.read().unwrap();
        let (provider_id, pm, _original) = config::resolve_model(&config, &req.model);

        let provider = match config.providers.get(provider_id) {
            Some(p) => p.clone(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": {"message": format!("未知 provider: {provider_id}"), "type": "invalid_request_error"}
                    })),
                )
                    .into_response();
            }
        };

        if provider.api_key.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": {"message": format!("Provider {provider_id} 未配置 API Key")}
                })),
            )
                .into_response();
        }

        provider_model = pm.to_string();
        base_url = provider.base_url().trim_end_matches('/').to_string();
        api_key = provider.api_key.clone();
    }
    client = state.http_client.clone();

    let chat_url = format!("{base_url}/chat/completions");
    let chat_req = convert::responses_to_chat(&req, &provider_model);

    let stream_result = client
        .post(&chat_url)
        .header("Authorization", format!("Bearer {api_key}"))
        .header("Content-Type", "application/json")
        .json(&chat_req)
        .send()
        .await;

    let response = match stream_result {
        Ok(r) => r,
        Err(e) => {
            error!("上游请求失败: {e}");
            return (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({
                    "error": {"message": format!("上游服务连接失败: {e}"), "type": "api_error"}
                })),
            )
                .into_response();
        }
    };

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        return (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({
                "error": {"message": format!("上游返回 {status}: {error_body}"), "type": "api_error"}
            })),
        )
            .into_response();
    }

    let mut stream = response.bytes_stream();
    let model_name = provider_model;

    let body_stream = async_stream::stream! {
        let mut converter = convert::SseConverter::new(&model_name);
        let mut responses_converter = convert::ResponsesSseConverter::new(&model_name);

        if let Some(init) = responses_converter.ensure_init() {
            for line in init.lines() {
                if !line.is_empty() {
                    yield Ok::<_, std::convert::Infallible>(format!("{line}\n"));
                }
            }
        }

        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    buffer.push_str(&text);

                    while let Some(line_end) = buffer.find('\n') {
                        let line = buffer[..line_end].trim().to_string();
                        buffer = buffer[line_end + 1..].to_string();

                        if line.is_empty() || line.starts_with(':') || line == "data: [DONE]" {
                            continue;
                        }

                        if let Some(data) = line.strip_prefix("data: ") {
                            if let Ok(chunk) = serde_json::from_str::<convert::OpenAISSEChunk>(data) {
                                if let Some(usage) = &chunk.usage {
                                    converter.update_usage(usage);
                                }
                                if let Some(choices) = &chunk.choices {
                                    for choice in choices {
                                        let anthropic_events = converter.process_choice(choice);
                                        for event in &anthropic_events {
                                            if let Some(json) = event.strip_prefix("data: ") {
                                                let resp_events = responses_converter.convert_anthropic_event(json);
                                                for re in resp_events {
                                                    yield Ok(re);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("SSE 读取错误: {e}");
                    break;
                }
            }
        }
    };

    let stream = tokio_stream::StreamExt::map(body_stream, |item| match item {
        Ok(s) => Ok::<_, std::convert::Infallible>(s),
    });

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .body(Body::from_stream(stream))
        .unwrap()
        .into_response()
}

// ==================== Models Endpoint ====================

async fn handle_models(State(state): State<SharedState>) -> Json<Value> {
    let config = state.config.read().unwrap();
    let mut models = Vec::new();

    for (provider_id, provider) in &config.providers {
        if provider.api_key.is_empty() {
            continue;
        }
        for model in &provider.models {
            models.push(serde_json::json!({
                "id": format!("{provider_id}/{model}"),
                "object": "model",
                "created": 0,
                "owned_by": provider.display_name,
            }));
        }
    }

    Json(serde_json::json!({
        "object": "list",
        "data": models
    }))
}

async fn handle_model_detail(
    axum::extract::Path(name): axum::extract::Path<String>,
) -> Json<Value> {
    Json(serde_json::json!({
        "id": name,
        "object": "model",
        "created": 0,
        "owned_by": "airelay"
    }))
}

// ==================== Health / Simple Endpoints ====================

async fn handle_health() -> &'static str {
    "ok"
}

async fn handle_count_tokens() -> Json<Value> {
    Json(serde_json::json!({
        "input_tokens": 0
    }))
}

// ==================== Admin Endpoints ====================

async fn handle_admin_ui() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "text/html; charset=utf-8")],
        admin_ui::HTML,
    )
}

async fn handle_admin_get_config(State(state): State<SharedState>) -> Json<Value> {
    let config = state.config.read().unwrap();
    let mut providers_json = serde_json::Map::new();

    for (id, p) in &config.providers {
        let mut pj = serde_json::json!({
            "display_name": p.display_name,
            "has_key": !p.api_key.is_empty(),
            "base_url": p.base_url(),
            "models": p.models,
        });
        if !p.api_key.is_empty() {
            pj["api_key_masked"] = serde_json::Value::String(mask_key(&p.api_key));
        }
        providers_json.insert(id.clone(), pj);
    }

    Json(serde_json::json!({
        "server": {
            "host": config.server.host,
            "port": config.server.port
        },
        "default": {
            "provider": config.default.provider,
            "model": config.default.model
        },
        "providers": providers_json
    }))
}

async fn handle_admin_update_config(
    State(state): State<SharedState>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let mut config = state.config.write().unwrap();

    if let Some(default) = body.get("default") {
        if let Some(provider) = default.get("provider").and_then(|v| v.as_str()) {
            config.default.provider = provider.to_string();
        }
        if let Some(model) = default.get("model").and_then(|v| v.as_str()) {
            config.default.model = model.to_string();
        }
    }

    if let Some(providers) = body.get("providers").and_then(|v| v.as_object()) {
        for (id, p_data) in providers {
            if let Some(provider) = config.providers.get_mut(id) {
                if let Some(key) = p_data.get("api_key").and_then(|v| v.as_str()) {
                    if !key.is_empty() && !is_masked(key) {
                        provider.api_key = key.to_string();
                    }
                }
                if let Some(url) = p_data.get("base_url").and_then(|v| v.as_str()) {
                    if !url.is_empty() {
                        provider.base_url = Some(url.to_string());
                    }
                }
                if let Some(models) = p_data.get("models").and_then(|v| v.as_array()) {
                    provider.models = models
                        .iter()
                        .filter_map(|m| m.as_str().map(|s| s.to_string()))
                        .collect();
                }
            }
        }
    }

    let path = state.config_path.read().unwrap().clone();
    config::save_config(&path, &config).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    info!("配置已更新并保存");
    Ok(Json(serde_json::json!({"ok": true})))
}

async fn handle_admin_create_provider(
    State(state): State<SharedState>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let id = body
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "缺少 provider id"})),
        ))?
        .to_string();

    if id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "id 不能为空"})),
        ));
    }

    let mut config = state.config.write().unwrap();

    if config.providers.contains_key(&id) {
        return Err((
            StatusCode::CONFLICT,
            Json(serde_json::json!({"error": format!("provider {id} 已存在")})),
        ));
    }

    let display_name = body
        .get("display_name")
        .and_then(|v| v.as_str())
        .unwrap_or(&id)
        .to_string();
    let api_key = body
        .get("api_key")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let base_url = body
        .get("base_url")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let models: Vec<String> = body
        .get("models")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|m| m.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();

    config.providers.insert(
        id.clone(),
        config::ProviderConfig {
            display_name,
            api_key,
            base_url,
            models,
        },
    );

    let path = state.config_path.read().unwrap().clone();
    config::save_config(&path, &config).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e})))
    })?;

    info!("新增 provider: {id}");
    Ok(Json(serde_json::json!({"ok": true})))
}

async fn handle_admin_delete_provider(
    State(state): State<SharedState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Compute fallback before acquiring write lock
    let fallback = {
        let config = state.config.read().unwrap();
        if config.default.provider == id && config.providers.len() > 1 {
            config
                .providers
                .iter()
                .find(|(k, _)| *k != &id)
                .map(|(k, v)| (k.clone(), v.models.first().cloned().unwrap_or_default()))
        } else {
            None
        }
    };

    let mut config = state.config.write().unwrap();

    if !config.providers.contains_key(&id) {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": format!("provider {id} 不存在")})),
        ));
    }

    config.providers.remove(&id);

    if config.default.provider == id {
        if let Some((fid, fmodel)) = fallback {
            config.default.provider = fid;
            config.default.model = fmodel;
        }
    }

    let path = state.config_path.read().unwrap().clone();
    config::save_config(&path, &config).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e})))
    })?;

    info!("删除 provider: {id}");
    Ok(Json(serde_json::json!({"ok": true})))
}

async fn handle_admin_test(
    State(state): State<SharedState>,
    Json(body): Json<Value>,
) -> Json<Value> {
    let provider_id = body
        .get("provider_id")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let api_key = body.get("api_key").and_then(|v| v.as_str()).unwrap_or("");
    let base_url = body.get("base_url").and_then(|v| v.as_str()).unwrap_or("");

    let key = if is_masked(api_key) || api_key.is_empty() {
        let config = state.config.read().unwrap();
        config
            .providers
            .get(provider_id)
            .map(|p| p.api_key.clone())
            .unwrap_or_default()
    } else {
        api_key.to_string()
    };

    if key.is_empty() {
        return Json(serde_json::json!({
            "ok": false,
            "error": "API Key 为空"
        }));
    }

    let url = format!("{}/models", base_url.trim_end_matches('/'));

    match state
        .http_client
        .get(&url)
        .header("Authorization", format!("Bearer {key}"))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {
            let body = resp.text().await.unwrap_or_default();
            let mut models = Vec::new();
            if let Ok(val) = serde_json::from_str::<Value>(&body) {
                if let Some(data) = val.get("data").and_then(|v| v.as_array()) {
                    for m in data {
                        if let Some(id) = m.get("id").and_then(|v| v.as_str()) {
                            if !id.contains("embed") && !id.contains("moderation") {
                                models.push(id.to_string());
                            }
                        }
                    }
                }
            }
            Json(serde_json::json!({
                "ok": true,
                "models": models,
                "message": format!("连接成功，发现 {} 个模型", models.len())
            }))
        }
        Ok(resp) => {
            let status = resp.status();
            Json(serde_json::json!({
                "ok": false,
                "error": format!("服务器返回 {status}")
            }))
        }
        Err(e) => Json(serde_json::json!({
            "ok": false,
            "error": format!("连接失败: {e}")
        })),
    }
}

// ==================== Helpers ====================

fn anthropic_error(error_type: &str, message: &str) -> Value {
    serde_json::json!({
        "type": "error",
        "error": {
            "type": error_type,
            "message": message
        }
    })
}

fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        return "****".to_string();
    }
    format!("{}****{}", &key[..4], &key[key.len() - 4..])
}

fn is_masked(value: &str) -> bool {
    value.contains("****")
}
