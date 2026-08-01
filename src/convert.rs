use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

// ==================== Anthropic Types ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesRequest {
    pub model: String,
    pub messages: Vec<AnthropicMessage>,
    #[serde(default)]
    pub system: Option<SystemContent>,
    pub max_tokens: u32,
    #[serde(default)]
    pub metadata: Option<Value>,
    #[serde(default)]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(default = "default_stream")]
    pub stream: bool,
    #[serde(default)]
    pub temperature: Option<f64>,
    #[serde(default)]
    pub top_p: Option<f64>,
    #[serde(default)]
    pub top_k: Option<u32>,
    #[serde(default)]
    pub tools: Option<Vec<AnthropicTool>>,
    #[serde(default)]
    pub thinking: Option<Value>,
}

fn default_stream() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicMessage {
    pub role: String,
    pub content: AnthropicContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnthropicContent {
    String(String),
    Blocks(Vec<ContentBlock>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text {
        text: String,
        #[serde(default)]
        citations: Option<Value>,
    },
    #[serde(rename = "image")]
    Image {
        source: ImageSource,
    },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: ToolResultContent,
        #[serde(default)]
        is_error: Option<bool>,
    },
    #[serde(rename = "thinking")]
    Thinking {
        thinking: String,
        #[serde(default)]
        signature: Option<String>,
    },
    #[serde(rename = "redacted_thinking")]
    RedactedThinking { data: String },
    #[serde(rename = "server_tool_use")]
    ServerToolUse {
        id: String,
        name: String,
        input: Value,
    },
    #[serde(rename = "web_search_tool_result")]
    WebSearchToolResult {
        #[serde(default)]
        content: Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolResultContent {
    String(String),
    Blocks(Vec<Value>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicTool {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub input_schema: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemContent {
    String(String),
    Blocks(Vec<Value>),
}

// ==================== OpenAI Types ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIChatRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<OpenAITool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIMessage {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<OpenAIMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<OpenAIToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAIMessageContent {
    String(String),
    Parts(Vec<OpenAIContentPart>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OpenAIContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: OpenAIImageUrl },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIImageUrl {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: OpenAIFunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIFunctionCall {
    #[serde(default)]
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAITool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: OpenAIFunctionDef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIFunctionDef {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub parameters: Value,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct OpenAISSEChunk {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub object: Option<String>,
    #[serde(default)]
    pub created: Option<i64>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub choices: Option<Vec<OpenAIStreamChoice>>,
    #[serde(default)]
    pub usage: Option<OpenAIUsage>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct OpenAIStreamChoice {
    #[serde(default)]
    pub index: i32,
    #[serde(default)]
    pub delta: OpenAIStreamDelta,
    #[serde(default)]
    pub finish_reason: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct OpenAIStreamDelta {
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<OpenAIToolCallDelta>>,
    #[serde(default)]
    pub reasoning_content: Option<String>,
    #[serde(default)]
    pub thinking: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenAIToolCallDelta {
    #[serde(default)]
    pub index: i32,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub function: Option<OpenAIFunctionCallDelta>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct OpenAIFunctionCallDelta {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub arguments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIUsage {
    #[serde(default)]
    pub prompt_tokens: u32,
    #[serde(default)]
    pub completion_tokens: u32,
    #[serde(default)]
    pub total_tokens: u32,
}

// ==================== OpenAI Responses Types ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsesRequest {
    pub model: String,
    pub input: Value,
    #[serde(default)]
    pub tools: Option<Value>,
    #[serde(default)]
    pub stream: Option<bool>,
    #[serde(default)]
    pub max_output_tokens: Option<u32>,
    #[serde(default)]
    pub temperature: Option<f64>,
}

// ==================== Conversion Functions ====================

pub fn anthropic_to_openai(req: &MessagesRequest, provider_model: &str) -> OpenAIChatRequest {
    let mut openai_messages: Vec<OpenAIMessage> = Vec::new();

    // Convert system prompt
    if let Some(ref sys) = req.system {
        let sys_text = match sys {
            SystemContent::String(s) => s.clone(),
            SystemContent::Blocks(blocks) => blocks
                .iter()
                .filter_map(|b| b.get("text").and_then(|t| t.as_str()))
                .collect::<Vec<_>>()
                .join("\n"),
        };
        if !sys_text.is_empty() {
            openai_messages.push(OpenAIMessage {
                role: "system".into(),
                content: Some(OpenAIMessageContent::String(sys_text)),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            });
        }
    }

    // Convert messages
    for msg in &req.messages {
        let openai_msg = convert_anthropic_message(msg);
        if let Some(m) = openai_msg {
            if let Some(ref prev) = openai_messages.last() {
                if prev.role == m.role
                    && matches!(m.content, Some(OpenAIMessageContent::String(_)))
                    && matches!(prev.content, Some(OpenAIMessageContent::String(_)))
                {
                    if let (
                        Some(OpenAIMessageContent::String(ref prev_text)),
                        Some(OpenAIMessageContent::String(ref new_text)),
                    ) = (&prev.content, &m.content)
                    {
                        let merged = OpenAIMessage {
                            role: prev.role.clone(),
                            content: Some(OpenAIMessageContent::String(format!(
                                "{prev_text}\n{new_text}"
                            ))),
                            tool_calls: None,
                            tool_call_id: None,
                            name: None,
                        };
                        openai_messages.pop();
                        openai_messages.push(merged);
                        continue;
                    }
                }
            }
            openai_messages.push(m);
        }
    }

    // Convert tools
    let openai_tools = req.tools.as_ref().map(|tools| {
        tools
            .iter()
            .map(|t| OpenAITool {
                tool_type: "function".into(),
                function: OpenAIFunctionDef {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    parameters: t.input_schema.clone(),
                },
            })
            .collect()
    });

    // Resolve reasoning effort
    let reasoning_effort = req.thinking.as_ref().and_then(|t| {
        t.get("type")
            .and_then(|v| v.as_str())
            .filter(|&s| s == "enabled")
            .and(
                t.get("budget_tokens")
                    .and_then(|v| v.as_u64())
                    .map(|tokens| {
                        if tokens >= 32000 {
                            "high"
                        } else if tokens >= 16000 {
                            "medium"
                        } else {
                            "low"
                        }
                    }),
            )
            .map(|s| s.to_string())
    });

    OpenAIChatRequest {
        model: provider_model.to_string(),
        messages: openai_messages,
        max_tokens: Some(req.max_tokens.min(32768)),
        temperature: req.temperature,
        top_p: req.top_p,
        stream: true,
        tools: openai_tools,
        reasoning_effort,
        stream_options: Some(serde_json::json!({"include_usage": true})),
        stop: req.stop_sequences.clone(),
    }
}

fn convert_anthropic_message(msg: &AnthropicMessage) -> Option<OpenAIMessage> {
    match &msg.content {
        AnthropicContent::String(text) => Some(OpenAIMessage {
            role: msg.role.clone(),
            content: Some(OpenAIMessageContent::String(text.clone())),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        }),
        AnthropicContent::Blocks(blocks) => convert_blocks_to_openai(&msg.role, blocks),
    }
}

fn convert_blocks_to_openai(role: &str, blocks: &[ContentBlock]) -> Option<OpenAIMessage> {
    let mut text_parts: Vec<String> = Vec::new();
    let mut content_parts: Vec<OpenAIContentPart> = Vec::new();
    let mut tool_calls: Vec<OpenAIToolCall> = Vec::new();
    let mut has_image = false;

    for block in blocks {
        match block {
            ContentBlock::Text { text, .. } => {
                text_parts.push(text.clone());
                content_parts.push(OpenAIContentPart::Text { text: text.clone() });
            }
            ContentBlock::Image { source } => {
                has_image = true;
                content_parts.push(OpenAIContentPart::ImageUrl {
                    image_url: OpenAIImageUrl {
                        url: format!("data:{};base64,{}", source.media_type, source.data),
                    },
                });
            }
            ContentBlock::ToolUse { id, name, input } => {
                tool_calls.push(OpenAIToolCall {
                    id: id.clone(),
                    call_type: "function".into(),
                    function: OpenAIFunctionCall {
                        name: name.clone(),
                        arguments: serde_json::to_string(input).unwrap_or_default(),
                    },
                });
            }
            ContentBlock::ToolResult {
                tool_use_id,
                content,
                ..
            } => {
                let ct = match content {
                    ToolResultContent::String(s) => s.clone(),
                    ToolResultContent::Blocks(b) => {
                        b.iter()
                            .filter_map(|v| v.get("text").and_then(|t| t.as_str()))
                            .collect::<Vec<_>>()
                            .join("\n")
                    }
                };
                return Some(OpenAIMessage {
                    role: "tool".into(),
                    content: Some(OpenAIMessageContent::String(ct)),
                    tool_calls: None,
                    tool_call_id: Some(tool_use_id.clone()),
                    name: None,
                });
            }
            ContentBlock::Thinking { thinking, .. } => {
                content_parts.push(OpenAIContentPart::Text {
                    text: thinking.clone(),
                });
            }
            ContentBlock::RedactedThinking { .. } => {
                // Skip redacted thinking — no content to forward
            }
            ContentBlock::ServerToolUse { id, name, input } => {
                tool_calls.push(OpenAIToolCall {
                    id: id.clone(),
                    call_type: "function".into(),
                    function: OpenAIFunctionCall {
                        name: name.clone(),
                        arguments: serde_json::to_string(input).unwrap_or_default(),
                    },
                });
            }
            ContentBlock::WebSearchToolResult { content } => {
                let search_text = match &content {
                    Value::String(s) => s.clone(),
                    Value::Array(arr) => arr
                        .iter()
                        .filter_map(|v| {
                            v.get("text")
                                .or(v.get("snippet"))
                                .or(v.get("title"))
                                .and_then(|t| t.as_str())
                        })
                        .collect::<Vec<_>>()
                        .join("\n"),
                    _ => serde_json::to_string(&content).unwrap_or_default(),
                };
                if !search_text.is_empty() {
                    text_parts.push(format!("[网页搜索结果]\n{search_text}"));
                }
            }
        }
    }

    if !tool_calls.is_empty() {
        Some(OpenAIMessage {
            role: "assistant".into(),
            content: None,
            tool_calls: Some(tool_calls),
            tool_call_id: None,
            name: None,
        })
    } else if role == "assistant" && !text_parts.is_empty() {
        Some(OpenAIMessage {
            role: "assistant".into(),
            content: Some(OpenAIMessageContent::String(text_parts.join("\n"))),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        })
    } else if !text_parts.is_empty() {
        if has_image {
            Some(OpenAIMessage {
                role: role.to_string(),
                content: Some(OpenAIMessageContent::Parts(content_parts)),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            })
        } else {
            Some(OpenAIMessage {
                role: role.to_string(),
                content: Some(OpenAIMessageContent::String(text_parts.join("\n"))),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            })
        }
    } else {
        None
    }
}

// ==================== SSE Conversion ====================

pub struct SseConverter {
    message_id: String,
    model: String,
    block_index: u32,
    text_block_open: bool,
    thinking_block_open: bool,
    sent_message_start: bool,
    tool_states: HashMap<i32, ToolState>,
    current_text_index: u32,
    current_thinking_index: u32,
    input_tokens: u32,
    output_tokens: u32,
}

struct ToolState {
    anthropic_index: u32,
    tool_id: String,
    name: String,
    arg_buffer: String,
    started: bool,
    arg_sent: bool,
}

impl SseConverter {
    pub fn new(model: &str) -> Self {
        Self {
            message_id: format!("msg_{}", Uuid::new_v4().to_string().replace('-', "")[..20].to_string()),
            model: model.to_string(),
            block_index: 0,
            text_block_open: false,
            thinking_block_open: false,
            sent_message_start: false,
            tool_states: HashMap::new(),
            current_text_index: 0,
            current_thinking_index: 0,
            input_tokens: 0,
            output_tokens: 0,
        }
    }

    pub fn update_usage(&mut self, usage: &OpenAIUsage) {
        if usage.prompt_tokens > 0 {
            self.input_tokens = usage.prompt_tokens;
        }
        if usage.completion_tokens > 0 {
            self.output_tokens = usage.completion_tokens;
        }
    }

    pub fn message_start(&self) -> String {
        format!(
            "event: message_start\ndata: {}\n\n",
            serde_json::json!({
                "type": "message_start",
                "message": {
                    "id": self.message_id,
                    "type": "message",
                    "role": "assistant",
                    "model": self.model,
                    "content": [],
                    "stop_reason": null,
                    "stop_sequence": null,
                    "usage": {"input_tokens": 0, "output_tokens": 0}
                }
            })
        )
    }

    pub fn ensure_message_start(&mut self) -> Option<String> {
        if !self.sent_message_start {
            self.sent_message_start = true;
            Some(self.message_start())
        } else {
            None
        }
    }

    pub fn process_choice(
        &mut self,
        choice: &OpenAIStreamChoice,
    ) -> Vec<String> {
        let mut events = Vec::new();

        // Handle thinking/reasoning content
        let thinking_text = choice
            .delta
            .reasoning_content
            .as_deref()
            .or(choice.delta.thinking.as_deref())
            .filter(|t| !t.is_empty());

        if let Some(think) = thinking_text {
            if self.text_block_open {
                events.push(self.close_text_block());
            }
            if !self.thinking_block_open {
                self.thinking_block_open = true;
                self.current_thinking_index = self.block_index;
                self.block_index += 1;
                events.push(format!(
                    "event: content_block_start\ndata: {}\n\n",
                    serde_json::json!({
                        "type": "content_block_start",
                        "index": self.current_thinking_index,
                        "content_block": {
                            "type": "thinking",
                            "thinking": "",
                            "signature": ""
                        }
                    })
                ));
            }
            events.push(format!(
                "event: content_block_delta\ndata: {}\n\n",
                serde_json::json!({
                    "type": "content_block_delta",
                    "index": self.current_thinking_index,
                    "delta": {
                        "type": "thinking_delta",
                        "thinking": think,
                        "signature": ""
                    }
                })
            ));
        }

        // Handle text content
        if let Some(ref text) = choice.delta.content {
            if !text.is_empty() {
                // Check for think tags in text (heuristic)
                let (think_part, text_part) = split_think_tags(text);

                if let Some(think) = think_part {
                    if self.text_block_open {
                        events.push(self.close_text_block());
                    }
                    if !self.thinking_block_open {
                        self.thinking_block_open = true;
                        self.current_thinking_index = self.block_index;
                        self.block_index += 1;
                        events.push(format!(
                            "event: content_block_start\ndata: {}\n\n",
                            serde_json::json!({
                                "type": "content_block_start",
                                "index": self.current_thinking_index,
                                "content_block": {
                                    "type": "thinking",
                                    "thinking": ""
                                }
                            })
                        ));
                    }
                    events.push(format!(
                        "event: content_block_delta\ndata: {}\n\n",
                        serde_json::json!({
                            "type": "content_block_delta",
                            "index": self.current_thinking_index,
                            "delta": {
                                "type": "thinking_delta",
                                "thinking": think
                            }
                        })
                    ));
                }

                if let Some(txt) = text_part {
                    if !txt.is_empty() {
                        if self.thinking_block_open {
                            events.push(self.close_thinking_block());
                        }
                        if !self.text_block_open {
                            self.text_block_open = true;
                            self.current_text_index = self.block_index;
                            self.block_index += 1;
                            events.push(format!(
                                "event: content_block_start\ndata: {}\n\n",
                                serde_json::json!({
                                    "type": "content_block_start",
                                    "index": self.current_text_index,
                                    "content_block": {
                                        "type": "text",
                                        "text": ""
                                    }
                                })
                            ));
                        }
                        events.push(format!(
                            "event: content_block_delta\ndata: {}\n\n",
                            serde_json::json!({
                                "type": "content_block_delta",
                                "index": self.current_text_index,
                                "delta": {
                                    "type": "text_delta",
                                    "text": txt
                                }
                            })
                        ));
                    }
                }
            }
        }

        // Handle tool calls
        if let Some(ref tool_calls) = choice.delta.tool_calls {
            for tc in tool_calls {
                if self.text_block_open {
                    events.push(self.close_text_block());
                }
                if self.thinking_block_open {
                    events.push(self.close_thinking_block());
                }

                let state = self.tool_states.entry(tc.index).or_insert_with(|| {
                    let idx = self.block_index;
                    self.block_index += 1;
                    ToolState {
                        anthropic_index: idx,
                        tool_id: tc.id.clone().unwrap_or_else(|| {
                            format!("toolu_{}", Uuid::new_v4().to_string().replace('-', "")[..12].to_string())
                        }),
                        name: String::new(),
                        arg_buffer: String::new(),
                        started: false,
                        arg_sent: false,
                    }
                });

                if let Some(ref id) = tc.id {
                    state.tool_id = id.clone();
                }

                if let Some(ref func) = tc.function {
                    if let Some(ref name) = func.name {
                        state.name = name.clone();
                    }
                    if let Some(ref args) = func.arguments {
                        state.arg_buffer.push_str(args);
                    }
                }

                if !state.started && !state.name.is_empty() {
                    state.started = true;
                    events.push(format!(
                        "event: content_block_start\ndata: {}\n\n",
                        serde_json::json!({
                            "type": "content_block_start",
                            "index": state.anthropic_index,
                            "content_block": {
                                "type": "tool_use",
                                "id": state.tool_id,
                                "name": state.name,
                                "input": {}
                            }
                        })
                    ));
                }

                // Try to emit argument deltas as they arrive
                if state.started && !state.arg_buffer.is_empty() && !state.arg_sent {
                    if let Ok(parsed) = serde_json::from_str::<Value>(&state.arg_buffer) {
                        if parsed.is_object() && !parsed.as_object().unwrap().is_empty() {
                            events.push(format!(
                                "event: content_block_delta\ndata: {}\n\n",
                                serde_json::json!({
                                    "type": "content_block_delta",
                                    "index": state.anthropic_index,
                                    "delta": {
                                        "type": "input_json_delta",
                                        "partial_json": state.arg_buffer
                                    }
                                })
                            ));
                            state.arg_sent = true;
                        }
                    }
                }
            }
        }

        // Handle finish reason
        if let Some(ref finish_reason) = choice.finish_reason {
            if self.text_block_open {
                events.push(self.close_text_block());
            }
            if self.thinking_block_open {
                events.push(self.close_thinking_block());
            }

            // Close any open tool blocks
            for state in self.tool_states.values() {
                if state.started && !state.arg_sent {
                    events.push(format!(
                        "event: content_block_delta\ndata: {}\n\n",
                        serde_json::json!({
                            "type": "content_block_delta",
                            "index": state.anthropic_index,
                            "delta": {
                                "type": "input_json_delta",
                                "partial_json": if state.arg_buffer.is_empty() { "{}" } else { &state.arg_buffer }
                            }
                        })
                    ));
                }
                if state.started {
                    events.push(format!(
                        "event: content_block_stop\ndata: {}\n\n",
                        serde_json::json!({
                            "type": "content_block_stop",
                            "index": state.anthropic_index
                        })
                    ));
                }
            }

            let stop_reason = match finish_reason.as_str() {
                "stop" => "end_turn",
                "length" => "max_tokens",
                "tool_calls" => "tool_use",
                _ => "end_turn",
            };

            // Try heuristic tool parsing from accumulated text
            // (for local models that embed tool calls in text output)
            events.push(format!(
                "event: message_delta\ndata: {}\n\n",
                serde_json::json!({
                    "type": "message_delta",
                    "delta": {
                        "stop_reason": stop_reason,
                        "stop_sequence": null
                    },
                    "usage": {"output_tokens": self.output_tokens}
                })
            ));

            events.push(format!(
                "event: message_stop\ndata: {}\n\n",
                serde_json::json!({
                    "type": "message_stop"
                })
            ));
        }

        events
    }

    fn close_text_block(&mut self) -> String {
        self.text_block_open = false;
        format!(
            "event: content_block_stop\ndata: {}\n\n",
            serde_json::json!({
                "type": "content_block_stop",
                "index": self.current_text_index
            })
        )
    }

    fn close_thinking_block(&mut self) -> String {
        self.thinking_block_open = false;
        format!(
            "event: content_block_stop\ndata: {}\n\n",
            serde_json::json!({
                "type": "content_block_stop",
                "index": self.current_thinking_index
            })
        )
    }
}

/// Split text into (thinking_part, text_part) by <think> tags
fn split_think_tags(text: &str) -> (Option<String>, Option<String>) {
    if text.contains("<think>") || text.contains("</think>") || text.contains("<｜end▁of▁thinking｜>") {
        let think_open = text.find("<think>");
        let think_close = text.find("</think>");

        match (think_open, think_close) {
            (Some(open), Some(close)) if close > open => {
                let think_content = &text[open + 8..close];
                let before = if open > 0 { &text[..open] } else { "" };
                let after = if close + 8 < text.len() { &text[close + 8..] } else { "" };
                let remaining = format!("{before}{after}").trim().to_string();
                (
                    Some(think_content.trim().to_string()),
                    if remaining.is_empty() {
                        None
                    } else {
                        Some(remaining)
                    },
                )
            }
            (Some(open), None) => {
                let think_content = &text[open + 8..];
                let before = if open > 0 { &text[..open] } else { "" };
                (
                    Some(think_content.trim().to_string()),
                    if before.is_empty() {
                        None
                    } else {
                        Some(before.to_string())
                    },
                )
            }
            (None, Some(close)) => {
                let think_content = &text[..close];
                let after = if close + 8 < text.len() { &text[close + 8..] } else { "" };
                (
                    Some(think_content.trim().to_string()),
                    if after.is_empty() {
                        None
                    } else {
                        Some(after.to_string())
                    },
                )
            }
            _ => (None, Some(text.to_string())),
        }
    } else {
        (None, Some(text.to_string()))
    }
}

/// Heuristic tool call parsing from text (fallback for local models)
#[allow(dead_code)]
fn try_parse_tool_calls(text: &str) -> Option<Vec<ContentBlock>> {
    let mut blocks = Vec::new();
    let mut remaining = text;

    while let Some(start) = remaining.find(r#""name""#) {
        let snippet = &remaining[start..];
        let end = snippet.find(r#""arguments""#);

        if let Some(args_pos) = end {
            let name_part = &snippet[..args_pos];
            let name = name_part
                .split('"')
                .nth(name_part.matches('"').count().saturating_sub(1))
                .unwrap_or("unknown");

            if let Some(args_start) = snippet[args_pos..].find('{') {
                let args_snippet = &snippet[args_pos + args_start..];
                let mut depth = 0;
                let mut args_end = 0;
                for (i, ch) in args_snippet.char_indices() {
                    match ch {
                        '{' => depth += 1,
                        '}' => {
                            depth -= 1;
                            if depth == 0 {
                                args_end = i + 1;
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                if args_end > 0 {
                    let args_str = &args_snippet[..args_end];
                    if let Ok(input) = serde_json::from_str::<Value>(args_str) {
                        blocks.push(ContentBlock::ToolUse {
                            id: format!("toolu_{}", Uuid::new_v4().to_string().replace('-', "")[..12].to_string()),
                            name: name.to_string(),
                            input,
                        });
                    }
                    remaining = &args_snippet[args_end..];
                    continue;
                }
            }
        }
        remaining = &remaining[1..];
    }

    if blocks.is_empty() {
        None
    } else {
        Some(blocks)
    }
}

// ==================== Responses API Conversion ====================

pub fn responses_to_chat(req: &ResponsesRequest, provider_model: &str) -> OpenAIChatRequest {
    let mut messages = Vec::new();

    // Parse input items into messages
    if let Some(items) = req.input.as_array() {
        for item in items {
            let role = item
                .get("role")
                .and_then(|v| v.as_str())
                .unwrap_or("user");
            let content = item
                .get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            if content.is_empty() {
                continue;
            }

            messages.push(OpenAIMessage {
                role: role.to_string(),
                content: Some(OpenAIMessageContent::String(content.to_string())),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            });
        }
    }

    // Parse tools from Responses format
    let openai_tools = req.tools.as_ref().and_then(|tools| {
        tools.as_array().map(|arr| {
            arr.iter()
                .filter_map(|t| {
                    let tool_type = t.get("type").and_then(|v| v.as_str())?;
                    match tool_type {
                        "function" => {
                            let name = t.get("name").and_then(|v| v.as_str())?.to_string();
                            let description = t
                                .get("description")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            let parameters = t
                                .get("parameters")
                                .cloned()
                                .unwrap_or(serde_json::json!({"type": "object", "properties": {}}));
                            Some(OpenAITool {
                                tool_type: "function".into(),
                                function: OpenAIFunctionDef {
                                    name,
                                    description,
                                    parameters,
                                },
                            })
                        }
                        _ => None,
                    }
                })
                .collect()
        })
    });

    OpenAIChatRequest {
        model: provider_model.to_string(),
        messages,
        max_tokens: req.max_output_tokens.map(|t| t.min(32768)),
        temperature: req.temperature,
        top_p: None,
        stream: true,
        tools: openai_tools,
        reasoning_effort: None,
        stream_options: Some(serde_json::json!({"include_usage": true})),
        stop: None,
    }
}

pub fn responses_to_anthropic(req: &ResponsesRequest, provider_model: &str) -> MessagesRequest {
    let mut messages: Vec<AnthropicMessage> = Vec::new();
    let mut system: Option<SystemContent> = None;

    if let Some(items) = req.input.as_array() {
        for item in items {
            let role = item
                .get("role")
                .and_then(|v| v.as_str())
                .unwrap_or("user");

            if role == "system" || role == "developer" {
                let content = item
                    .get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if !content.is_empty() {
                    system = Some(SystemContent::String(content.to_string()));
                }
                continue;
            }

            let content = item.get("content");
            if let Some(content) = content {
                if let Some(text) = content.as_str() {
                    if !text.is_empty() {
                        messages.push(AnthropicMessage {
                            role: role.to_string(),
                            content: AnthropicContent::String(text.to_string()),
                        });
                    }
                } else if let Some(arr) = content.as_array() {
                    let blocks: Vec<ContentBlock> = arr
                        .iter()
                        .filter_map(|part| {
                            let part_type = part.get("type").and_then(|v| v.as_str())?;
                            match part_type {
                                "input_text" | "text" => {
                                    let text = part.get("text").and_then(|v| v.as_str())?;
                                    Some(ContentBlock::Text {
                                        text: text.to_string(),
                                        citations: None,
                                    })
                                }
                                "input_image" | "image" => {
                                    let url = part
                                        .get("image_url")
                                        .or(part.get("url"))
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("");
                                    let media_type = if url.starts_with("data:") {
                                        url.split(';')
                                            .next()
                                            .and_then(|s| s.strip_prefix("data:"))
                                            .unwrap_or("image/png")
                                            .to_string()
                                    } else {
                                        "image/png".to_string()
                                    };
                                    let data = if url.starts_with("data:") {
                                        url.split(',')
                                            .nth(1)
                                            .unwrap_or("")
                                            .to_string()
                                    } else {
                                        url.to_string()
                                    };
                                    Some(ContentBlock::Image {
                                        source: ImageSource {
                                            source_type: "base64".into(),
                                            media_type,
                                            data,
                                        },
                                    })
                                }
                                _ => None,
                            }
                        })
                        .collect();
                    if !blocks.is_empty() {
                        messages.push(AnthropicMessage {
                            role: role.to_string(),
                            content: AnthropicContent::Blocks(blocks),
                        });
                    }
                }
            }
        }
    }

    let anthropic_tools = req.tools.as_ref().and_then(|tools| {
        tools.as_array().map(|arr| {
            arr.iter()
                .filter_map(|t| {
                    let tool_type = t.get("type").and_then(|v| v.as_str())?;
                    match tool_type {
                        "function" => {
                            let name = t.get("name").and_then(|v| v.as_str())?.to_string();
                            let description = t
                                .get("description")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            let input_schema = t
                                .get("parameters")
                                .cloned()
                                .unwrap_or(serde_json::json!({"type": "object", "properties": {}}));
                            Some(AnthropicTool {
                                name,
                                description,
                                input_schema,
                            })
                        }
                        _ => None,
                    }
                })
                .collect()
        })
    });

    MessagesRequest {
        model: provider_model.to_string(),
        messages,
        system,
        max_tokens: req.max_output_tokens.unwrap_or(4096),
        metadata: None,
        stop_sequences: None,
        stream: req.stream.unwrap_or(true),
        temperature: req.temperature,
        top_p: None,
        top_k: None,
        tools: anthropic_tools,
        thinking: None,
    }
}

/// Convert Chat SSE event string to Responses SSE event string
pub struct ResponsesSseConverter {
    response_id: String,
    model: String,
    sent_created: bool,
    text_item_id: Option<String>,
    output_tokens: u32,
}

impl ResponsesSseConverter {
    pub fn new(model: &str) -> Self {
        Self {
            response_id: format!("resp_{}", Uuid::new_v4().to_string().replace('-', "")[..20].to_string()),
            model: model.to_string(),
            sent_created: false,
            text_item_id: None,
            output_tokens: 0,
        }
    }

    pub fn ensure_init(&mut self) -> Option<String> {
        if !self.sent_created {
            self.sent_created = true;
            let mut events = String::new();

            events.push_str(&format!(
                "event: response.created\ndata: {}\n\n",
                serde_json::json!({
                    "type": "response.created",
                    "response": {
                        "id": self.response_id,
                        "object": "response",
                        "model": self.model,
                        "status": "in_progress",
                        "output": []
                    }
                })
            ));

            Some(events)
        } else {
            None
        }
    }

    /// Convert an Anthropic SSE event line to Responses SSE
    pub fn convert_anthropic_event(&mut self, sse_data: &str) -> Vec<String> {
        let value: Value = match serde_json::from_str(sse_data) {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };

        let mut events = Vec::new();

        match value.get("type").and_then(|v| v.as_str()) {
            Some("message_start") => {
                events.extend(self.ensure_init());
            }

            Some("content_block_start") => {
                let block = value.get("content_block");
                if let Some(block) = block {
                    match block.get("type").and_then(|v| v.as_str()) {
                        Some("text") => {
                            self.text_item_id = Some(format!(
                                "item_{}",
                                Uuid::new_v4().to_string().replace('-', "")[..16].to_string()
                            ));
                            events.push(format!(
                                "event: response.output_item.added\ndata: {}\n\n",
                                serde_json::json!({
                                    "type": "response.output_item.added",
                                    "item": {
                                        "id": self.text_item_id.as_ref().unwrap(),
                                        "type": "message",
                                        "role": "assistant",
                                        "status": "in_progress",
                                        "content": []
                                    }
                                })
                            ));
                            events.push(format!(
                                "event: response.content_part.added\ndata: {}\n\n",
                                serde_json::json!({
                                    "type": "response.content_part.added",
                                    "item_id": self.text_item_id.as_ref().unwrap(),
                                    "part": {
                                        "type": "output_text",
                                        "text": ""
                                    }
                                })
                            ));
                        }
                        Some("tool_use") => {
                            let tool_id = block.get("id").and_then(|v| v.as_str()).unwrap_or("");
                            let tool_name = block.get("name").and_then(|v| v.as_str()).unwrap_or("");
                            events.push(format!(
                                "event: response.output_item.added\ndata: {}\n\n",
                                serde_json::json!({
                                    "type": "response.output_item.added",
                                    "item": {
                                        "id": tool_id,
                                        "type": "function_call",
                                        "name": tool_name,
                                        "status": "in_progress",
                                        "arguments": ""
                                    }
                                })
                            ));
                        }
                        _ => {}
                    }
                }
            }

            Some("content_block_delta") => {
                let delta = value.get("delta");
                if let Some(delta) = delta {
                    match delta.get("type").and_then(|v| v.as_str()) {
                        Some("text_delta") => {
                            if let Some(text) = delta.get("text").and_then(|v| v.as_str()) {
                                if let Some(ref item_id) = self.text_item_id {
                                    events.push(format!(
                                        "event: response.output_text.delta\ndata: {}\n\n",
                                        serde_json::json!({
                                            "type": "response.output_text.delta",
                                            "item_id": item_id,
                                            "delta": text
                                        })
                                    ));
                                }
                            }
                        }
                        Some("input_json_delta") => {
                            if let Some(json_str) =
                                delta.get("partial_json").and_then(|v| v.as_str())
                            {
                                let index = value.get("index").and_then(|v| v.as_u64()).unwrap_or(0);
                                let tool_id = format!("item_tool_{index}");
                                events.push(format!(
                                    "event: response.function_call.delta\ndata: {}\n\n",
                                    serde_json::json!({
                                        "type": "response.function_call.delta",
                                        "item_id": tool_id,
                                        "arguments": json_str
                                    })
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }

            Some("content_block_stop") => {
                // No action needed
            }

            Some("message_delta") => {
                let _stop_reason = value
                    .get("delta")
                    .and_then(|d| d.get("stop_reason"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("end_turn");

                if let Some(u) = value.get("usage").and_then(|u| u.get("output_tokens")).and_then(|v| v.as_u64()) {
                    self.output_tokens = u as u32;
                }

                events.push(format!(
                    "event: response.completed\ndata: {}\n\n",
                    serde_json::json!({
                        "type": "response.completed",
                        "response": {
                            "id": self.response_id,
                            "object": "response",
                            "model": self.model,
                            "status": "completed",
                            "output": [],
                            "usage": {
                                "input_tokens": 0,
                                "output_tokens": self.output_tokens,
                                "total_tokens": self.output_tokens
                            }
                        }
                    })
                ));
            }

            _ => {}
        }

        events
    }
}
