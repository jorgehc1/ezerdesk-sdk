use serde::{Deserialize, Serialize};
pub use ezerdesk_sdk_macros::main;

#[derive(Serialize, Deserialize, Debug)]
pub struct PluginResponse {
    pub success: bool,
    pub ui_widgets: Vec<UiWidget>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionResponse {
    pub success: bool,
    pub response: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavItem {
    pub page_id: String,
    pub label: String,
    pub icon: String,
    pub category: String,
    pub priority: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PluginMetadata {
    pub navigation: Vec<NavItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "tipo", content = "data")]
pub enum UiWidget {
    #[serde(rename = "card")]
    Card { title: String, children: Vec<UiWidget> },
    #[serde(rename = "text")]
    Text { content: String, style: String },
    #[serde(rename = "button")]
    Button { label: String, action: String, variant: String },
    #[serde(rename = "input")]
    Input { label: String, name: String, placeholder: String, value: String },
    #[serde(rename = "textarea")]
    Textarea { label: String, name: String, placeholder: String, value: String },
    #[serde(rename = "select")]
    Select { label: String, name: String, options: Vec<(String, String)>, value: String },
    #[serde(rename = "switch")]
    Switch { label: String, name: String, value: bool },
    #[serde(rename = "badge")]
    Badge { content: String, variant: String },
    #[serde(rename = "icon")]
    Icon { name: String, color: String },
    #[serde(rename = "divider")]
    Divider,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event_type")]
pub enum PluginEvent {
    #[serde(rename = "get_ui_fragments")]
    GetUiFragments { location: String },
    #[serde(rename = "page_request")]
    PageRequest { page_id: String },
    #[serde(rename = "plugin_action")]
    PluginAction { action: String, data: serde_json::Value },
    #[serde(rename = "ticket.created")]
    TicketCreated {
        id: String,
        asunto: String,
        #[serde(default)]
        id_departamento: Option<String>,
        #[serde(default)]
        id_solicitante: Option<String>,
        #[serde(default)]
        descripcion: Option<String>,
    },
    #[serde(rename = "plugin.updated")]
    PluginUpdated {
        id_organizacion: String,
        id_plugin: String,
    },
    #[serde(rename = "get_metadata")]
    GetMetadata,
    #[serde(other)]
    Other,
}

// Host Imports
#[link(wasm_import_module = "env")]
extern "C" {
    fn print(ptr: *const u8, len: u32);
    fn host_kv_set(k_ptr: *const u8, k_len: u32, v_ptr: *const u8, v_len: u32);
    fn host_kv_read(k_ptr: *const u8, k_len: u32, buf_ptr: *mut u8, buf_len: u32) -> u32;
}

pub fn log(msg: &str) {
    unsafe { print(msg.as_ptr(), msg.len() as u32) };
}

pub fn kv_set_val(key: &str, value: &str) {
    unsafe { host_kv_set(key.as_ptr(), key.len() as u32, value.as_ptr(), value.len() as u32) };
}

pub fn kv_get_val(key: &str) -> Option<String> {
    let mut buf = [0u8; 4096];
    let actual_len = unsafe { host_kv_read(key.as_ptr(), key.len() as u32, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    
    if actual_len > 0 && actual_len <= 4096 {
        Some(String::from_utf8_lossy(&buf[0..actual_len]).to_string())
    } else {
        None
    }
}

pub fn to_host_response<T: Serialize>(response: &T) {
    let json = serde_json::to_string(response).unwrap_or_else(|_| "{}".to_string());
    log(&json);
}

// Memory management helpers (Internal)
pub fn allocate(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

pub fn deallocate(ptr: *mut u8, size: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, size);
    }
}
