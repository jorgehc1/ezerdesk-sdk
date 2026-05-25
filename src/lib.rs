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
    pub host: String,
    pub navigation: Vec<NavItem>,
    /// Nombre del plugin (se usará como nombre en el sistema al subirlo)
    #[serde(default)]
    pub name: Option<String>,
    /// Descripción del plugin
    #[serde(default)]
    pub description: Option<String>,
    /// Versión semántica del plugin (ej: "1.0.0")
    #[serde(default)]
    pub version: Option<String>,
    /// Autor o empresa que desarrolló el plugin
    #[serde(default)]
    pub author: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "tipo", content = "data")]
pub enum UiWidget {
    #[serde(rename = "card")]
    Card {
        title: String,
        children: Vec<UiWidget>,
        #[serde(default)]
        colspan: Option<i32>,
    },
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
    #[serde(rename = "modal")]
    Modal {
        title: String,
        children: Vec<UiWidget>,
        size: String,
        close_action: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    pub id: String,
    pub id_organizacion: String,
    pub asunto: String,
    pub descripcion: String,
    pub id_departamento: String,
    pub id_solicitante: String,
    pub prioridad: String,
    pub estado: String,
    #[serde(default)]
    pub nuevo_estado: Option<String>,
    #[serde(default)]
    pub nueva_prioridad: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    pub id: String,
    pub id_organizacion: String,
    pub id_ticket: String,
    pub id_autor: String,
    pub cuerpo: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub id: String,
    pub id_sesion: String,
    pub id_organizacion: String,
    pub remitente: String,
    pub contenido: String,
    pub metadatos: String, // JSON string as per backend codec
    pub creado_en: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatSession {
    pub id_sesion: String,
    pub id_organizacion: String,
    pub id_usuario: String,
    pub estado: String,
    pub id_agente: Option<String>,
    pub puntaje_sentiment: f32,
    pub etiqueta_sentiment: String,
    pub creado_at: i64,
    pub actualizado_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "event_type")]
pub enum PluginEvent {
    // ── Eventos de Tickets ──────────────────────────────────────────────────
    #[serde(rename = "ticket.created")]
    TicketCreated(Ticket),

    #[serde(rename = "ticket.updated")]
    TicketUpdated(Ticket),

    #[serde(rename = "ticket.status.changed")]
    TicketStatusChanged(Ticket),

    #[serde(rename = "ticket.priority.changed")]
    TicketPriorityChanged(Ticket),

    #[serde(rename = "ticket.deleted")]
    TicketDeleted {
        id_organizacion: String,
        id_ticket: String,
    },

    #[serde(rename = "comment.added")]
    CommentAdded(Comment),

    // ── Eventos de Chat ─────────────────────────────────────────────────────
    #[serde(rename = "chat.message.created")]
    ChatMessageCreated(ChatMessage),

    #[serde(rename = "session.created")]
    SessionCreated(ChatSession),

    #[serde(rename = "session.closed")]
    SessionClosed {
        id_sesion: String,
        id_organizacion: String,
    },

    #[serde(rename = "chat.sentiment.alert")]
    ChatSentimentAlert {
        id_sesion: String,
        id_organizacion: String,
        puntaje: f32,
        etiqueta: String,
        racha: i32,
    },

    #[serde(rename = "handover.requested")]
    HandoverRequested {
        id_sesion: String,
        id_organizacion: String,
        senales: Vec<String>,
    },

    // ── Eventos de IAM ──────────────────────────────────────────────────────
    #[serde(rename = "agent.created")]
    AgentCreated {
        id_organizacion: String,
        id_usuario: String,
    },

    #[serde(rename = "agent.deleted")]
    AgentDeleted {
        id_organizacion: String,
        id_usuario: String,
    },

    // ── Eventos de Flujos de Trabajo (Workflows) ───────────────────────────
    #[serde(rename = "workflow.created")]
    WorkflowCreated {
        id_organizacion: String,
        id_workflow: String,
    },

    #[serde(rename = "workflow.updated")]
    WorkflowUpdated {
        id_organizacion: String,
        id_workflow: String,
    },

    #[serde(rename = "workflow.deleted")]
    WorkflowDeleted {
        id_organizacion: String,
        id_workflow: String,
    },

    #[serde(rename = "workflow.triggered")]
    WorkflowTriggered {
        id_organizacion: String,
        id_workflow: String,
        id_ticket: String,
    },

    // ── Eventos de Sistema / Plugins ────────────────────────────────────────
    #[serde(rename = "plugin.updated")]
    PluginUpdated {
        id_organizacion: String,
        id_plugin: String,
    },

    #[serde(rename = "plugin.deleted")]
    PluginDeleted {
        id_organizacion: String,
        id_plugin: String,
    },

    // ── Eventos de SLA ───────────────────────────────────────────────────────
    #[serde(rename = "sla.breached")]
    SlaBreached {
        id_organizacion: String,
        id_ticket: String,
        vence_en: i64,
    },

    #[serde(rename = "get_metadata")]
    GetMetadata,

    #[serde(rename = "get_ui_fragments")]
    GetUiFragments {
        location: String,
    },

    #[serde(rename = "page_request")]
    PageRequest {
        page_id: String,
    },

    #[serde(rename = "plugin_action")]
    PluginAction {
        action: String,
        data: serde_json::Value,
    },

    #[serde(other)]
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub body: String,
    pub headers: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

// Host Imports
#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn host_publish_response(ptr: *const u8, len: u32);
    fn host_kv_set(k_ptr: *const u8, k_len: u32, v_ptr: *const u8, v_len: u32);
    fn host_kv_read(k_ptr: *const u8, k_len: u32, buf_ptr: *mut u8, buf_len: u32) -> u32;
    fn host_http_request(req_ptr: *const u8, req_len: u32, res_ptr: *mut u8, res_len: u32) -> u32;
}

pub fn log(msg: &str) {
    unsafe { host_publish_response(msg.as_ptr(), msg.len() as u32) };
}

pub fn http_request(req: &HttpRequest) -> Option<HttpResponse> {
    let json = serde_json::to_string(req).ok()?;
    let mut buf = [0u8; 8192]; // Buffer para la respuesta (8KB)
    
    let actual_len = unsafe { 
        host_http_request(json.as_ptr(), json.len() as u32, buf.as_mut_ptr(), buf.len() as u32) 
    } as usize;

    if actual_len > 0 && actual_len <= 8192 {
        let res_json = String::from_utf8_lossy(&buf[0..actual_len]).to_string();
        serde_json::from_str(&res_json).ok()
    } else {
        None
    }
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
