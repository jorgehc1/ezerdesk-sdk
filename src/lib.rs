use serde::{Deserialize, Serialize};
pub use ezerdesk_sdk_macros::main;

#[derive(Serialize, Deserialize, Debug)]
pub struct PluginResponse {
    pub success: bool,
    #[serde(default)]
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
    #[serde(default)]
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

// Host Imports — solo activos en WASM; stubs para poder compilar tests nativos
#[cfg(target_arch = "wasm32")]
mod host {
    #[link(wasm_import_module = "env")]
    unsafe extern "C" {
        pub fn host_publish_response(ptr: *const u8, len: u32);
        pub fn host_kv_set(k_ptr: *const u8, k_len: u32, v_ptr: *const u8, v_len: u32);
        pub fn host_kv_read(k_ptr: *const u8, k_len: u32, buf_ptr: *mut u8, buf_len: u32) -> u32;
        pub fn host_http_request(req_ptr: *const u8, req_len: u32, res_ptr: *mut u8, res_len: u32) -> u32;
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod host {
    pub unsafe fn host_publish_response(_ptr: *const u8, _len: u32) {}
    pub unsafe fn host_kv_set(_k_ptr: *const u8, _k_len: u32, _v_ptr: *const u8, _v_len: u32) {}
    pub unsafe fn host_kv_read(_k_ptr: *const u8, _k_len: u32, _buf_ptr: *mut u8, _buf_len: u32) -> u32 { 0 }
    pub unsafe fn host_http_request(_req_ptr: *const u8, _req_len: u32, _res_ptr: *mut u8, _res_len: u32) -> u32 { 0 }
}

use host::*;

pub fn log(msg: &str) {
    unsafe { host_publish_response(msg.as_ptr(), msg.len() as u32) };
}

pub fn http_request(req: &HttpRequest) -> Option<HttpResponse> {
    let json = match serde_json::to_string(req) {
        Ok(j) => j,
        Err(_) => return None,
    };
    let mut buf = [0u8; 65536]; // Buffer para la respuesta (64KB)

    let actual_len = unsafe {
        host_http_request(json.as_ptr(), json.len() as u32, buf.as_mut_ptr(), buf.len() as u32)
    } as usize;

    if actual_len == 0 {
        return None;
    }

    if actual_len > buf.len() {
        log(&format!(
            "[SDK] http_request: response too large ({} bytes, max {})",
            actual_len,
            buf.len()
        ));
        return None;
    }

    let res_json = String::from_utf8_lossy(&buf[0..actual_len]).to_string();
    serde_json::from_str(&res_json).ok()
}

pub fn kv_set_val(key: &str, value: &str) {
    unsafe { host_kv_set(key.as_ptr(), key.len() as u32, value.as_ptr(), value.len() as u32) };
}

pub fn kv_get_val(key: &str) -> Option<String> {
    let mut buf = [0u8; 16384]; // Buffer para lectura (16KB)
    let actual_len = unsafe {
        host_kv_read(key.as_ptr(), key.len() as u32, buf.as_mut_ptr(), buf.len() as u32)
    } as usize;

    if actual_len == 0 {
        return None;
    }

    if actual_len > buf.len() {
        log(&format!(
            "[SDK] kv_get_val: value too large for key '{}' ({} bytes, max {})",
            key,
            actual_len,
            buf.len()
        ));
        return None;
    }

    Some(String::from_utf8_lossy(&buf[0..actual_len]).to_string())
}

pub fn to_host_response<T: Serialize>(response: &T) {
    match serde_json::to_string(response) {
        Ok(json) => log(&json),
        Err(e) => log(&format!("[SDK] Error serializing response: {}", e)),
    }
}

// Memory management helpers (Internal)
pub fn allocate(size: usize) -> *mut u8 {
    let actual = if size == 0 { 1 } else { size };
    let mut buf = Vec::with_capacity(actual);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

pub fn deallocate(ptr: *mut u8, size: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Serialización de PluginMetadata ──────────────────────────────────────

    #[test]
    fn test_plugin_metadata_roundtrip() {
        let meta = PluginMetadata {
            host: "ezerdesk".to_string(),
            navigation: vec![NavItem {
                page_id: "main".to_string(),
                label: "Mi Plugin".to_string(),
                icon: "rocket".to_string(),
                category: "ops".to_string(),
                priority: 10,
            }],
            name: Some("Test".to_string()),
            description: Some("A test plugin".to_string()),
            version: Some("1.0.0".to_string()),
            author: Some("RFJ".to_string()),
        };
        let json = serde_json::to_string(&meta).unwrap();
        let deserialized: PluginMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.host, meta.host);
        assert_eq!(deserialized.navigation.len(), 1);
        assert_eq!(deserialized.name.unwrap(), "Test");
    }

    #[test]
    fn test_plugin_metadata_default_host() {
        let json = r#"{"navigation":[]}"#;
        let meta: PluginMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(meta.host, "");
        assert!(meta.navigation.is_empty());
    }

    #[test]
    fn test_plugin_metadata_all_optionals_absent() {
        let json = r#"{"host":"ezerdesk","navigation":[]}"#;
        let meta: PluginMetadata = serde_json::from_str(json).unwrap();
        assert!(meta.name.is_none());
        assert!(meta.description.is_none());
        assert!(meta.version.is_none());
        assert!(meta.author.is_none());
    }

    // ── Serialización de PluginResponse ──────────────────────────────────────

    #[test]
    fn test_plugin_response_ui_widgets_default() {
        let json = r#"{"success":true}"#;
        let resp: PluginResponse = serde_json::from_str(json).unwrap();
        assert!(resp.success);
        assert!(resp.ui_widgets.is_empty());
    }

    #[test]
    fn test_plugin_response_roundtrip() {
        let resp = PluginResponse {
            success: true,
            ui_widgets: vec![
                UiWidget::Card {
                    title: "Card".to_string(),
                    children: vec![UiWidget::Text { content: "Hello".to_string(), style: "info".to_string() }],
                    colspan: None,
                },
            ],
        };
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: PluginResponse = serde_json::from_str(&json).unwrap();
        assert!(deserialized.success);
        assert_eq!(deserialized.ui_widgets.len(), 1);
    }

    // ── Serialización de UiWidget ────────────────────────────────────────────

    #[test]
    fn test_ui_widget_card_roundtrip() {
        let widget = UiWidget::Card {
            title: "Dashboard".to_string(),
            children: vec![],
            colspan: Some(2),
        };
        let json = serde_json::to_string(&widget).unwrap();
        let deserialized: UiWidget = serde_json::from_str(&json).unwrap();
        match deserialized {
            UiWidget::Card { title, colspan, .. } => {
                assert_eq!(title, "Dashboard");
                assert_eq!(colspan, Some(2));
            }
            _ => panic!("Expected Card"),
        }
    }

    #[test]
    fn test_ui_widget_all_variants_roundtrip() {
        let variants: Vec<UiWidget> = vec![
            UiWidget::Card { title: "c".into(), children: vec![], colspan: None },
            UiWidget::Text { content: "t".into(), style: "info".into() },
            UiWidget::Button { label: "b".into(), action: "act".into(), variant: "primary".into() },
            UiWidget::Input { label: "l".into(), name: "n".into(), placeholder: "p".into(), value: "v".into() },
            UiWidget::Textarea { label: "l".into(), name: "n".into(), placeholder: "p".into(), value: "v".into() },
            UiWidget::Select { label: "l".into(), name: "n".into(), options: vec![("k".into(), "v".into())], value: "k".into() },
            UiWidget::Switch { label: "l".into(), name: "n".into(), value: true },
            UiWidget::Badge { content: "c".into(), variant: "success".into() },
            UiWidget::Icon { name: "star".into(), color: "yellow".into() },
            UiWidget::Divider,
            UiWidget::Modal { title: "m".into(), children: vec![], size: "lg".into(), close_action: "close".into() },
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant).unwrap();
            let deserialized: UiWidget = serde_json::from_str(&json).unwrap();
            assert_eq!(
                std::mem::discriminant(variant),
                std::mem::discriminant(&deserialized),
                "Mismatch for variant"
            );
        }
    }

    // ── Serialización de PluginEvent ─────────────────────────────────────────

    #[test]
    fn test_plugin_event_get_metadata() {
        let event = PluginEvent::GetMetadata;
        let json = serde_json::to_string(&event).unwrap();
        let deserialized: PluginEvent = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized, PluginEvent::GetMetadata));
    }

    #[test]
    fn test_plugin_event_ticket_created() {
        let ticket = Ticket {
            id: "t1".into(),
            id_organizacion: "org1".into(),
            asunto: "Issue".into(),
            descripcion: "desc".into(),
            id_departamento: "dept1".into(),
            id_solicitante: "user1".into(),
            prioridad: "alta".into(),
            estado: "abierto".into(),
            nuevo_estado: None,
            nueva_prioridad: None,
        };
        let event = PluginEvent::TicketCreated(ticket);
        let json = serde_json::to_string(&event).unwrap();
        let deserialized: PluginEvent = serde_json::from_str(&json).unwrap();
        match deserialized {
            PluginEvent::TicketCreated(t) => {
                assert_eq!(t.id, "t1");
                assert_eq!(t.asunto, "Issue");
            }
            _ => panic!("Expected TicketCreated"),
        }
    }

    #[test]
    fn test_plugin_event_other_unknown() {
        let json = r#"{"event_type":"unknown_event"}"#;
        let event: PluginEvent = serde_json::from_str(json).unwrap();
        assert!(matches!(event, PluginEvent::Other));
    }

    #[test]
    fn test_plugin_event_roundtrip_all() {
        let events = vec![
            PluginEvent::GetMetadata,
            PluginEvent::GetUiFragments { location: "dashboard".into() },
            PluginEvent::PageRequest { page_id: "main".into() },
            PluginEvent::PluginAction { action: "submit".into(), data: serde_json::json!({"key": "val"}) },
            PluginEvent::TicketCreated(Ticket {
                id: "t1".into(), id_organizacion: "o".into(), asunto: "a".into(),
                descripcion: "d".into(), id_departamento: "dd".into(), id_solicitante: "s".into(),
                prioridad: "p".into(), estado: "e".into(), nuevo_estado: None, nueva_prioridad: None,
            }),
        ];
        for event in &events {
            let json = serde_json::to_string(event).unwrap();
            let deserialized: PluginEvent = serde_json::from_str(&json).unwrap();
            assert_eq!(
                std::mem::discriminant(event),
                std::mem::discriminant(&deserialized)
            );
        }
    }

    // ── allocate / deallocate ─────────────────────────────────────────────────

    #[test]
    fn test_allocate_zero_returns_non_null() {
        let ptr = allocate(0);
        assert!(!ptr.is_null());
        deallocate(ptr, 1);
    }

    #[test]
    fn test_allocate_and_deallocate() {
        let ptr = allocate(64);
        assert!(!ptr.is_null());
        unsafe {
            std::ptr::write_bytes(ptr, 0xAB, 64);
        }
        deallocate(ptr, 64);
    }

    // ── HttpRequest / HttpResponse ────────────────────────────────────────────

    #[test]
    fn test_http_request_roundtrip() {
        let req = HttpRequest {
            method: "GET".into(),
            url: "https://example.com/api".into(),
            body: "".into(),
            headers: vec![("Authorization".into(), "Bearer xyz".into())],
        };
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: HttpRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.method, "GET");
        assert_eq!(deserialized.url, "https://example.com/api");
        assert_eq!(deserialized.headers.len(), 1);
    }

    // ── ActionResponse ───────────────────────────────────────────────────────

    #[test]
    fn test_action_response_roundtrip() {
        let resp = ActionResponse { success: true, response: "ok".into() };
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: ActionResponse = serde_json::from_str(&json).unwrap();
        assert!(deserialized.success);
        assert_eq!(deserialized.response, "ok");
    }
}
