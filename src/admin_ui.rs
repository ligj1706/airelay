pub const HTML: &str = r##"<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>airelay</title>
<style>
*,*::before,*::after{box-sizing:border-box;margin:0;padding:0}
:root{
  --bg:#F9F8F6;
  --card:#FFFFFF;
  --text:#1A1A18;
  --text2:#6B645C;
  --text3:#99938B;
  --border:#E8E4DE;
  --input-border:#D4CFC8;
  --focus:#D97746;
  --focus-ring:rgba(217,119,70,0.22);
  --btn-bg:#D97746;
  --btn-hover:#C56A3C;
  --btn-text:#FFFFFF;
  --green:#3D8B5E;
  --red:#C44E4E;
  --code-bg:#F4F1ED;
  --radius:6px;
  --shadow:0 1px 3px rgba(0,0,0,0.05);
  --font:-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,"Helvetica Neue",Arial,"Noto Sans SC",sans-serif;
  --mono:"SF Mono","Fira Code","Fira Mono","Roboto Mono",monospace;
}
@media (prefers-color-scheme:dark){
  :root:not([data-theme="light"]){
    --bg:#1B1A17;--card:#24221E;--text:#E8E4DE;--text2:#A69E93;--text3:#6B645C;
    --border:#3A3733;--input-border:#4A4742;--focus:#E8844F;--focus-ring:rgba(232,132,79,0.22);
    --btn-bg:#D97746;--btn-hover:#C56A3C;--btn-text:#FFFFFF;--code-bg:#2D2A26;
    --green:#4EA86E;--red:#D97676;--shadow:0 1px 3px rgba(0,0,0,0.2)
  }
}
[data-theme="dark"]{
  --bg:#1B1A17;--card:#24221E;--text:#E8E4DE;--text2:#A69E93;--text3:#6B645C;
  --border:#3A3733;--input-border:#4A4742;--focus:#E8844F;--focus-ring:rgba(232,132,79,0.22);
  --btn-bg:#D97746;--btn-hover:#C56A3C;--btn-text:#FFFFFF;--code-bg:#2D2A26;
  --green:#4EA86E;--red:#D97676;--shadow:0 1px 3px rgba(0,0,0,0.2)
}
body{background:var(--bg);color:var(--text);font-family:var(--font);line-height:1.5;-webkit-font-smoothing:antialiased}
header{border-bottom:1px solid var(--border);background:var(--card);padding:0 20px}
header .wrap{max-width:580px;margin:0 auto;display:flex;align-items:center;justify-content:space-between;height:48px}
header .logo{font-size:14px;font-weight:600;letter-spacing:-0.01em}
header .status{display:flex;align-items:center;gap:12px;font-size:11px;color:var(--text2)}
header .dot{width:6px;height:6px;border-radius:50%;background:var(--green);flex-shrink:0}
.container{max-width:580px;margin:0 auto;padding:20px}

.section{margin-bottom:20px}
.section-title{font-size:12px;font-weight:600;color:var(--text2);text-transform:uppercase;letter-spacing:0.04em;margin-bottom:8px}
.card{background:var(--card);border:1px solid var(--border);border-radius:var(--radius);padding:16px;box-shadow:var(--shadow)}

select,input[type="text"],input[type="password"]{
  width:100%;padding:8px 10px;border:1px solid var(--input-border);border-radius:var(--radius);
  font-size:13px;font-family:var(--font);background:var(--card);color:var(--text);
  outline:none;transition:border-color .15s,box-shadow .15s;-webkit-appearance:none;appearance:none
}
select{background-image:url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath d='M3 5l3 3 3-3' fill='none' stroke='%2399938B' stroke-width='1.5' stroke-linecap='round'/%3E%3C/svg%3E");background-repeat:no-repeat;background-position:right 10px center;padding-right:28px}
input:focus,select:focus{border-color:var(--focus);box-shadow:0 0 0 3px var(--focus-ring)}
input::placeholder{color:var(--text3)}
textarea{width:100%;padding:8px 10px;border:1px solid var(--input-border);border-radius:var(--radius);font-size:13px;font-family:var(--mono);background:var(--card);color:var(--text);outline:none;resize:vertical;min-height:32px}
textarea:focus{border-color:var(--focus);box-shadow:0 0 0 3px var(--focus-ring)}

.btn{display:inline-flex;align-items:center;justify-content:center;gap:4px;padding:7px 16px;border:none;border-radius:var(--radius);font-size:12px;font-weight:500;cursor:pointer;transition:background .15s,opacity .15s;font-family:var(--font);white-space:nowrap}
.btn:active{transform:scale(0.98)}
.btn-primary{background:var(--btn-bg);color:var(--btn-text)}
.btn-primary:hover{background:var(--btn-hover)}
.btn-outline{background:transparent;color:var(--focus);border:1px solid var(--focus)}
.btn-outline:hover{background:rgba(217,119,70,0.06)}
.btn-ghost{background:transparent;color:var(--text2);border:1px solid var(--border)}
.btn-ghost:hover{background:var(--bg)}
.btn-danger{background:var(--red);color:#fff}
.btn-danger:hover{opacity:0.85}
.btn-sm{padding:4px 10px;font-size:11px}

.toggle{position:relative;display:inline-block;width:44px;height:24px;flex-shrink:0}
.toggle input{opacity:0;width:0;height:0}
.toggle-slider{position:absolute;cursor:pointer;top:0;left:0;right:0;bottom:0;background:var(--border);border-radius:24px;transition:.2s}
.toggle-slider:before{position:absolute;content:"";height:18px;width:18px;left:3px;bottom:3px;background:white;border-radius:50%;transition:.2s}
.toggle input:checked+.toggle-slider{background:var(--focus)}
.toggle input:checked+.toggle-slider:before{transform:translateX(20px)}

.form-row{display:flex;gap:8px;align-items:flex-end}
.form-row .field{flex:1}
.field{margin-bottom:12px}
.field:last-child{margin-bottom:0}
.field label{display:block;font-size:11px;font-weight:500;color:var(--text2);margin-bottom:4px}
.field .hint{font-size:10px;color:var(--text3);margin-top:2px}

.msg{font-size:11px;padding:6px 10px;border-radius:4px;margin-top:8px;display:none}
.msg.show{display:block}
.msg.ok{background:#EDF5F0;color:var(--green)}
.msg.err{background:#FDF0EF;color:var(--red)}
@media (prefers-color-scheme:dark){
  :root:not([data-theme="light"]) .msg.ok{background:#1C3126;color:#4EA86E}
  :root:not([data-theme="light"]) .msg.err{background:#331C1D;color:#D97676}
}
[data-theme="dark"] .msg.ok{background:#1C3126;color:#4EA86E}
[data-theme="dark"] .msg.err{background:#331C1D;color:#D97676}

.provider-dots{display:flex;flex-wrap:wrap;gap:6px}
.provider-dot{
  display:inline-flex;align-items:center;gap:4px;padding:4px 10px;
  font-size:11px;border-radius:100px;cursor:pointer;transition:background .15s;
  border:1px solid var(--border);background:var(--card)
}
.provider-dot:hover{background:var(--code-bg)}
.provider-dot .d{width:5px;height:5px;border-radius:50%;flex-shrink:0}
.provider-dot .d.on{background:var(--green)}
.provider-dot .d.off{background:var(--text3)}
.provider-dot .d.local{background:#8FA89B}

.flex-between{display:flex;align-items:center;justify-content:space-between}
.gap-8{display:flex;gap:8px;align-items:center}
.mt-8{margin-top:8px}
.mt-12{margin-top:12px}

footer{text-align:center;padding:16px;font-size:10px;color:var(--text3);border-top:1px solid var(--border);margin-top:20px}
</style>
</head>
<body>
<header>
  <div class="wrap">
    <div class="logo">airelay</div>
    <div class="status"><span class="dot" id="statusDot"></span><span id="statusText">运行中</span><button class="btn btn-ghost btn-sm" onclick="toggleTheme()" id="themeBtn">暗色</button></div>
  </div>
</header>

<div class="container">

<!-- Active Model Selector -->
<div class="section">
  <div class="section-title">当前模型</div>
  <div class="card">
    <div class="form-row">
      <div class="field" style="margin-bottom:0">
        <label>提供商</label>
        <select id="activeProvider" onchange="onActiveProviderChange()"></select>
      </div>
      <div class="field" style="margin-bottom:0">
        <label>模型</label>
        <select id="activeModel"></select>
      </div>
    </div>
    <div class="hint mt-8" style="font-size:11px">Claude Code 中可使用 <code>provider/model</code> 格式切换，或直接使用默认模型</div>
  </div>
</div>

<!-- Auto-start Toggle -->
<div class="section">
  <div class="section-title">系统</div>
  <div class="card">
    <div class="flex-between">
      <div>
        <div style="font-size:13px;font-weight:500">开机自启</div>
        <div class="hint">登录时自动启动 airelay（macOS）</div>
      </div>
      <label class="toggle">
        <input type="checkbox" id="autostartToggle" onchange="toggleAutostart()">
        <span class="toggle-slider"></span>
      </label>
    </div>
    <div class="msg mt-8" id="autostartMsg"></div>
  </div>
</div>

<!-- Provider Configuration -->
<div class="section">
  <div class="section-title">配置提供商</div>
  <div class="card">
    <div class="flex-between" style="margin-bottom:12px">
      <div class="form-row" style="flex:1">
        <div class="field" style="margin-bottom:0">
          <label>选择提供商</label>
          <select id="configProvider" onchange="onConfigProviderChange()"></select>
        </div>
        <div class="field" style="margin-bottom:0">
          <label style="visibility:hidden">.</label>
          <button class="btn btn-outline btn-sm" onclick="testCurrent()">测试连接</button>
        </div>
      </div>
      <button class="btn btn-ghost btn-sm" onclick="showCreateForm()" style="margin-left:8px;flex-shrink:0;align-self:flex-end">+ 新增</button>
    </div>

    <div id="createForm" style="display:none;margin-bottom:14px;padding:12px;background:var(--code-bg);border-radius:var(--radius)">
      <div class="field"><label>Provider ID（唯一标识）</label><input type="text" id="newProviderId" placeholder="my-provider"></div>
      <div class="field"><label>显示名称</label><input type="text" id="newDisplayName" placeholder="我的提供商"></div>
      <div class="field"><label>Base URL</label><input type="text" id="newBaseUrl" placeholder="https://api.example.com/v1"></div>
      <div class="field"><label>API Key</label><input type="password" id="newApiKey" placeholder="sk-xxxxxxxx"></div>
      <div class="field"><label>模型列表（逗号分隔）</label><input type="text" id="newModels" placeholder="model-v1, model-v2"></div>
      <div class="flex-between mt-12">
        <button class="btn btn-primary btn-sm" onclick="doCreateProvider()">创建提供商</button>
        <button class="btn btn-ghost btn-sm" onclick="hideCreateForm()">取消</button>
      </div>
      <div class="msg" id="createMsg"></div>
    </div>

    <div id="configForm" style="display:none;margin-top:14px;padding-top:14px;border-top:1px solid var(--border)">
      <div class="field">
        <label>API Key <a href="#" id="cfgApiKeyUrl" target="_blank" style="font-weight:500;font-size:12px;color:var(--focus);text-decoration:none;margin-left:8px;border-bottom:1px dashed var(--focus);padding-bottom:1px">申请 Key →</a></label>
        <input type="password" id="cfgApiKey" placeholder="sk-xxxxxxxx" autocomplete="off">
        <div class="hint" id="cfgKeyHint"></div>
      </div>
      <div class="field">
        <label>Base URL</label>
        <input type="text" id="cfgBaseUrl">
      </div>
      <div class="field">
        <label>模型列表（逗号分隔）</label>
        <textarea id="cfgModels" rows="2"></textarea>
      </div>

      <div class="flex-between mt-12">
        <button class="btn btn-primary" onclick="saveCurrent()">保存此提供商</button>
        <div class="gap-8">
          <button class="btn btn-ghost btn-sm" onclick="saveAll()">保存全部</button>
          <button class="btn btn-danger btn-sm" onclick="deleteProvider()">删除</button>
        </div>
      </div>
      <div class="msg" id="cfgMsg"></div>
    </div>
  </div>
</div>

<!-- Provider Status -->
<div class="section">
  <div class="section-title">提供商状态 <span style="font-weight:400;text-transform:none;letter-spacing:0;font-size:10px;color:var(--text3)">（点击切换配置）</span></div>
  <div class="provider-dots" id="providerDots"></div>
</div>

</div>

<footer>airelay · API Key 仅存本地 <code>~/.airelay/config.toml</code></footer>

<script>
let state = {providers:{},default:{}};
let editingId = null;

const API_KEY_URLS = {
  deepseek:'https://platform.deepseek.com/api_keys',
  kimi:'https://platform.moonshot.cn/console/api-keys',
  glm:'https://open.bigmodel.cn/usercenter/apikeys',
  minimax:'https://platform.minimax.io/user-center/basic-information/interface-key',
  qwen:'https://bailian.console.aliyun.com/?apiKey=1',
  openai:'https://platform.openai.com/api-keys',
  ollama:'',
  lmstudio:'',
  custom:''
};

async function load(){
  const r = await fetch('/admin/api/config');
  state = await r.json();
  editingId = state.default.provider;
  renderAll();
}

function renderAll(){
  renderActiveSelector();
  renderConfigSelector();
  renderConfigForm();
  renderDots();
}

function renderActiveSelector(){
  const ap = document.getElementById('activeProvider');
  ap.innerHTML = '';
  for(const [id,p] of Object.entries(state.providers)){
    const opt = document.createElement('option');
    opt.value = id;
    opt.textContent = p.display_name + (p.has_key ? '' : ' (未配置)');
    if(state.default.provider === id) opt.selected = true;
    ap.appendChild(opt);
  }
  renderModelOptions();
}

function renderModelOptions(){
  const pid = document.getElementById('activeProvider').value;
  const am = document.getElementById('activeModel');
  am.innerHTML = '';
  if(pid && state.providers[pid]){
    for(const m of state.providers[pid].models||[]){
      const opt = document.createElement('option');
      opt.value = m; opt.textContent = m;
      if(state.default.provider === pid && state.default.model === m) opt.selected = true;
      am.appendChild(opt);
    }
  }
}

function onActiveProviderChange(){
  renderModelOptions();
  saveDefault();
}

async function saveDefault(){
  const pid = document.getElementById('activeProvider').value;
  const mid = document.getElementById('activeModel').value;
  if(!pid || !mid) return;
  state.default = {provider:pid, model:mid};
  await fetch('/admin/api/config', {method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({default:{provider:pid,model:mid}})});
}

function renderConfigSelector(){
  const sel = document.getElementById('configProvider');
  sel.innerHTML = '';
  for(const [id,p] of Object.entries(state.providers)){
    const opt = document.createElement('option');
    opt.value = id;
    const icon = p.base_url && (p.base_url.includes('localhost')||p.base_url.includes('127.0.0.1')) ? '◎' : (p.has_key ? '●' : '○');
    opt.textContent = icon + ' ' + p.display_name;
    if(id === editingId) opt.selected = true;
    sel.appendChild(opt);
  }
}

function renderConfigForm(){
  const id = document.getElementById('configProvider').value;
  editingId = id;
  const p = state.providers[id];
  if(!p) { document.getElementById('configForm').style.display = 'none'; return; }
  document.getElementById('configForm').style.display = 'block';

  const isLocal = p.base_url && (p.base_url.includes('localhost')||p.base_url.includes('127.0.0.1'));

  document.getElementById('cfgApiKey').value = '';
  document.getElementById('cfgApiKey').placeholder = isLocal ? '本地服务无需 Key' : (p.has_key ? (p.api_key_masked||'已保存') : 'sk-xxxxxxxx');
  document.getElementById('cfgKeyHint').textContent = isLocal ? '本地模型不需要 API Key' : (p.has_key ? '已保存，输入新 Key 将覆盖' : '');

  const keyUrl = API_KEY_URLS[id];
  const linkEl = document.getElementById('cfgApiKeyUrl');
  if(keyUrl){
    linkEl.href = keyUrl;
    linkEl.style.display = 'inline';
  } else {
    linkEl.style.display = 'none';
  }

  document.getElementById('cfgBaseUrl').value = p.base_url || '';
  document.getElementById('cfgModels').value = (p.models||[]).join(', ');
}

function onConfigProviderChange(){ renderConfigForm(); }

function renderDots(){
  const div = document.getElementById('providerDots');
  div.innerHTML = '';
  for(const [id,p] of Object.entries(state.providers)){
    const isLocal = p.base_url && (p.base_url.includes('localhost')||p.base_url.includes('127.0.0.1'));
    const cl = isLocal ? 'local' : (p.has_key ? 'on' : 'off');
    const dot = document.createElement('span');
    dot.className = 'provider-dot';
    dot.innerHTML = '<span class="d '+cl+'"></span>'+p.display_name;
    dot.title = p.has_key ? '已配置' : '未配置';
    dot.onclick = function(){
      document.getElementById('configProvider').value = id;
      renderConfigForm();
    };
    div.appendChild(dot);
  }
}

async function testCurrent(){
  const id = document.getElementById('configProvider').value;
  const key = document.getElementById('cfgApiKey').value;
  const url = document.getElementById('cfgBaseUrl').value || state.providers[id]?.base_url || '';
  if(!key && !state.providers[id]?.has_key){ showMsg('cfgMsg','err','请先输入 API Key'); return; }
  showMsg('cfgMsg','ok','测试中…');
  const r = await fetch('/admin/api/test', {method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({provider_id:id,api_key:key,base_url:url})});
  const d = await r.json();
  if(d.ok){
    if(d.models && d.models.length>0) document.getElementById('cfgModels').value = d.models.join(', ');
    showMsg('cfgMsg','ok',d.message);
  } else {
    showMsg('cfgMsg','err',d.error||'连接失败');
  }
}

async function saveCurrent(){
  const id = document.getElementById('configProvider').value;
  const keyEl = document.getElementById('cfgApiKey');
  const urlEl = document.getElementById('cfgBaseUrl');
  const modelsEl = document.getElementById('cfgModels');
  const keyVal = keyEl.value.trim();
  const urlVal = urlEl.value.trim();
  const models = modelsEl.value.split(',').map(s=>s.trim()).filter(s=>s);

  const body = { default: state.default, providers: { [id]: { api_key: keyVal||'', base_url: urlVal, models: models } } };

  if(keyVal) state.providers[id].has_key = true;
  if(keyVal) state.providers[id].api_key_masked = '****'+keyVal.slice(-4);

  const r = await fetch('/admin/api/config', {method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(body)});
  if(r.ok){ await load(); showMsg('cfgMsg','ok','配置已保存'); }
  else { showMsg('cfgMsg','err','保存失败'); }
}

async function saveAll(){
  const body = { default: state.default, providers: {} };
  for(const [id] of Object.entries(state.providers)){
    if(id === editingId){
      const keyVal = document.getElementById('cfgApiKey').value.trim();
      const urlVal = document.getElementById('cfgBaseUrl').value.trim();
      const modelsRaw = document.getElementById('cfgModels').value;
      const models = modelsRaw.split(',').map(s=>s.trim()).filter(s=>s);
      body.providers[id] = { api_key: keyVal||'', base_url: urlVal, models: models };
    } else {
      body.providers[id] = { models: state.providers[id].models };
    }
  }
  const r = await fetch('/admin/api/config', {method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(body)});
  if(r.ok){ await load(); showMsg('cfgMsg','ok','全部已保存'); }
  else { showMsg('cfgMsg','err','保存失败'); }
}

// Theme
function toggleTheme(){
  const cur = document.documentElement.getAttribute('data-theme');
  const next = cur === 'dark' ? 'light' : 'dark';
  document.documentElement.setAttribute('data-theme', next);
  document.getElementById('themeBtn').textContent = next === 'dark' ? '亮色' : '暗色';
  try{ localStorage.setItem('airelay-theme', next); }catch(e){}
}

function initTheme(){
  let t;
  try{ t = localStorage.getItem('airelay-theme'); }catch(e){}
  if(!t && window.matchMedia('(prefers-color-scheme: dark)').matches) t = 'dark';
  if(t){ document.documentElement.setAttribute('data-theme', t); document.getElementById('themeBtn').textContent = t === 'dark' ? '亮色' : '暗色'; }
}

// Provider add/delete
function showCreateForm(){ document.getElementById('createForm').style.display = 'block'; }

function hideCreateForm(){
  document.getElementById('createForm').style.display = 'none';
  document.getElementById('newProviderId').value = '';
  document.getElementById('newDisplayName').value = '';
  document.getElementById('newBaseUrl').value = '';
  document.getElementById('newApiKey').value = '';
  document.getElementById('newModels').value = '';
}

async function doCreateProvider(){
  const id = document.getElementById('newProviderId').value.trim();
  const dn = document.getElementById('newDisplayName').value.trim();
  const url = document.getElementById('newBaseUrl').value.trim();
  const key = document.getElementById('newApiKey').value.trim();
  const models = document.getElementById('newModels').value.split(',').map(s=>s.trim()).filter(s=>s);
  if(!id){ showMsg('createMsg','err','Provider ID 不能为空'); return; }
  showMsg('createMsg','ok','创建中…');
  const r = await fetch('/admin/api/provider', {method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({id:id,display_name:dn||id,base_url:url,api_key:key,models:models})});
  const d = await r.json();
  if(r.ok){ hideCreateForm(); await load(); showMsg('cfgMsg','ok','提供商已创建'); }
  else { showMsg('createMsg','err',d.error||'创建失败'); }
}

async function deleteProvider(){
  const id = document.getElementById('configProvider').value;
  if(!confirm('确定删除提供商 '+id+' ？此操作不可撤销。')) return;
  const r = await fetch('/admin/api/provider/'+encodeURIComponent(id), {method:'DELETE'});
  if(r.ok){ await load(); showMsg('cfgMsg','ok','已删除'); }
  else { const d = await r.json(); showMsg('cfgMsg','err',d.error||'删除失败'); }
}

function showMsg(id,type,text){
  const el = document.getElementById(id);
  if(!el) return;
  el.textContent = text;
  el.className = 'msg show '+type;
  setTimeout(()=>{el.className='msg'}, 4000);
}

async function loadAutostart(){
  try{
    const r = await fetch('/admin/api/autostart');
    const d = await r.json();
    document.getElementById('autostartToggle').checked = d.enabled;
  }catch(e){}
}

async function toggleAutostart(){
  const enable = document.getElementById('autostartToggle').checked;
  try{
    const r = await fetch('/admin/api/autostart', {method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({enable})});
    const d = await r.json();
    if(d.ok){
      showMsg('autostartMsg','ok', enable ? '开机自启已启用' : '开机自启已关闭');
    }
  }catch(e){
    showMsg('autostartMsg','err','操作失败');
    document.getElementById('autostartToggle').checked = !enable;
  }
}

initTheme();

document.getElementById('activeProvider').addEventListener('change', onActiveProviderChange);
document.getElementById('activeModel').addEventListener('change', saveDefault);
document.getElementById('configProvider').addEventListener('change', onConfigProviderChange);

load();
loadAutostart();
setInterval(async()=>{
  try{
    const r = await fetch('/health');
    if(r.ok){
      document.getElementById('statusDot').style.background='var(--green)';
      document.getElementById('statusText').textContent='运行中';
    } else { throw new Error(); }
  } catch(e) {
    document.getElementById('statusDot').style.background='var(--red)';
    document.getElementById('statusText').textContent='离线';
  }
}, 10000);
</script>
</body>
</html>"##;
