use serde::{Deserialize, Serialize};
pub use ezerdesk_sdk_macros::main;

pub mod query;

/// Respuesta del plugin al host, que puede incluir widgets de UI.
#[derive(Serialize, Deserialize, Debug)]
pub struct PluginResponse {
    /// Indica si la operación fue exitosa.
    pub success: bool,
    /// Widgets de UI a renderizar en el frontend.
    #[serde(default)]
    pub ui_widgets: Vec<UiWidget>,
}

/// Respuesta de acción simple (éxito/error con mensaje).
#[derive(Serialize, Deserialize, Debug)]
pub struct ActionResponse {
    /// Indica si la operación fue exitosa.
    pub success: bool,
    /// Mensaje de respuesta.
    pub response: String,
}

/// Elemento de navegación del plugin en el panel lateral.
#[derive(Serialize, Deserialize, Debug)]
pub struct NavItem {
    /// Identificador único de la página.
    pub page_id: String,
    /// Etiqueta visible en el menú.
    pub label: String,
    /// Nombre del ícono (ej: "rocket-line").
    pub icon: String,
    /// Categoría de navegación para agrupar items.
    pub category: String,
    /// Prioridad de ordenamiento (menor = más arriba).
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

/// Representa un widget de interfaz de usuario renderizable en el frontend.
///
/// Cada variante se serializa con el tag `"tipo"` para que el frontend
/// pueda interpretar el tipo de widget dinámicamente.
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
    #[serde(rename = "table")]
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
        #[serde(default)]
        caption: Option<String>,
    },
    #[serde(rename = "chart")]
    Chart {
        title: String,
        data: Vec<(String, f64)>,
        chart_type: String,
    },
    #[serde(rename = "number_input")]
    NumberInput {
        label: String,
        name: String,
        placeholder: String,
        value: String,
        min: Option<f64>,
        max: Option<f64>,
        step: Option<f64>,
    },
    #[serde(rename = "date_input")]
    DateInput {
        label: String,
        name: String,
        placeholder: String,
        value: String,
    },
    #[serde(rename = "calendar")]
    Calendar {
        label: String,
        name: String,
        selected_date: String,
        events: Vec<CalendarEvent>,
    },
    #[serde(rename = "file_upload")]
    FileUpload {
        label: String,
        name: String,
        accept: String,
        max_size: Option<i32>,
    },
    #[serde(rename = "rich_text")]
    RichText {
        label: String,
        name: String,
        value: String,
        placeholder: String,
    },
}

/// Evento del calendario
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarEvent {
    pub date: String,
    pub title: String,
    pub color: String,
}

/// Representa un ticket del sistema de helpdesk.
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

/// Comentario asociado a un ticket.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    pub id: String,
    pub id_organizacion: String,
    pub id_ticket: String,
    pub id_autor: String,
    pub cuerpo: String,
}

/// Mensaje de chat en una sesión de soporte en vivo.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub id: String,
    pub id_sesion: String,
    pub id_organizacion: String,
    pub remitente: String,
    pub contenido: String,
    /// Metadatos adicionales en formato JSON (según codec del backend).
    pub metadatos: String,
    pub creado_en: i64,
}

/// Sesión de chat de soporte en vivo.
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

/// Evento entrante del host hacia el plugin.
///
/// Se discrimina por el campo `event_type` en el JSON.
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

/// Petición HTTP para enviar al backend a través del host.
#[derive(Serialize, Deserialize, Debug)]
pub struct HttpRequest {
    /// Método HTTP (GET, POST, PUT, DELETE, etc.).
    pub method: String,
    /// URL completa del destino.
    pub url: String,
    /// Cuerpo de la petición.
    pub body: String,
    /// Cabeceras HTTP como pares (nombre, valor).
    pub headers: Vec<(String, String)>,
}

/// Respuesta HTTP recibida del backend.
#[derive(Serialize, Deserialize, Debug)]
pub struct HttpResponse {
    /// Código de estado HTTP.
    pub status: u16,
    /// Cuerpo de la respuesta.
    pub body: String,
}

// Host Imports — solo activos en WASM; stubs para poder compilar tests nativos
#[cfg(target_arch = "wasm32")]
mod host {
    #[link(wasm_import_module = "env")]
    unsafe extern "C" {
        pub fn host_publish_response(ptr: *const u8, len: u32);
        pub fn host_log(ptr: *const u8, len: u32);
        pub fn host_kv_set(k_ptr: *const u8, k_len: u32, v_ptr: *const u8, v_len: u32);
        pub fn host_kv_read(k_ptr: *const u8, k_len: u32, buf_ptr: *mut u8, buf_len: u32) -> u32;
        pub fn host_http_request(req_ptr: *const u8, req_len: u32, res_ptr: *mut u8, res_len: u32) -> u32;
        pub fn host_query(req_ptr: *const u8, req_len: u32, res_ptr: *mut u8, res_len: u32) -> u32;
        pub fn host_oauth_start(p_ptr: *const u8, p_len: u32, r_ptr: *mut u8, r_len: u32) -> u32;
        pub fn host_oauth_callback(c_ptr: *const u8, c_len: u32, b_ptr: *mut u8, b_len: u32) -> u32;
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod host {
    pub unsafe fn host_publish_response(_ptr: *const u8, _len: u32) {}
    pub unsafe fn host_log(_ptr: *const u8, _len: u32) {}
    pub unsafe fn host_kv_set(_k_ptr: *const u8, _k_len: u32, _v_ptr: *const u8, _v_len: u32) {}
    pub unsafe fn host_kv_read(_k_ptr: *const u8, _k_len: u32, _buf_ptr: *mut u8, _buf_len: u32) -> u32 { 0 }
    pub unsafe fn host_http_request(_req_ptr: *const u8, _req_len: u32, _res_ptr: *mut u8, _res_len: u32) -> u32 { 0 }
    pub unsafe fn host_query(_req_ptr: *const u8, _req_len: u32, _res_ptr: *mut u8, _res_len: u32) -> u32 { 0 }
    pub unsafe fn host_oauth_start(_p_ptr: *const u8, _p_len: u32, _r_ptr: *mut u8, _r_len: u32) -> u32 { 0 }
    pub unsafe fn host_oauth_callback(_c_ptr: *const u8, _c_len: u32, _b_ptr: *mut u8, _b_len: u32) -> u32 { 0 }
}

use host::*;

const INITIAL_BUF_SIZE: usize = 65536;
const KV_BUF_SIZE: usize = 16384;
const MAX_RETRIES: u32 = 5;

/// Envía un mensaje de log al host del plugin.
/// Los logs NO sobrescriben la respuesta del plugin (usan host_log separado).
pub fn log(msg: &str) {
    unsafe { host_log(msg.as_ptr(), msg.len() as u32) };
}

/// Realiza una petición HTTP al backend a través del host.
///
/// Usa un buffer dinámico con reintento automático si la respuesta no cabe.
pub fn http_request(req: &HttpRequest) -> Option<HttpResponse> {
    let json = match serde_json::to_string(req) {
        Ok(j) => j,
        Err(e) => {
            log(&format!("[SDK] http_request: failed to serialize request: {}", e));
            return None;
        }
    };

    let mut buf_size = INITIAL_BUF_SIZE;
    for _ in 0..MAX_RETRIES {
        let mut buf = vec![0u8; buf_size];
        let written = unsafe {
            host_http_request(
                json.as_ptr(),
                json.len() as u32,
                buf.as_mut_ptr(),
                buf.len() as u32,
            )
        } as usize;

        if written == 0 {
            log("[SDK] http_request: host returned empty response");
            return None;
        }

        if written > buf.len() {
            buf_size = written;
            continue;
        }

        if written == buf.len() {
            log(&format!(
                "[SDK] http_request: response may have been truncated (filled {}/{} buffer)",
                written, buf_size
            ));
        }

        return match serde_json::from_slice(&buf[..written]) {
            Ok(res) => Some(res),
            Err(e) => {
                log(&format!("[SDK] http_request: failed to deserialize response: {}", e));
                None
            }
        };
    }

    log(&format!(
        "[SDK] http_request: exceeded max retries ({}) for buffer allocation",
        MAX_RETRIES
    ));
    None
}

/// Ejecuta una consulta de datos contra el host mediante un JSON de query.
///
/// Usa un buffer dinámico con reintento automático si la respuesta no cabe.
pub fn query_data(query_json: &str) -> Option<String> {
    let mut buf_size = INITIAL_BUF_SIZE;
    for _ in 0..MAX_RETRIES {
        let mut buf = vec![0u8; buf_size];
        let written = unsafe {
            host_query(
                query_json.as_ptr(),
                query_json.len() as u32,
                buf.as_mut_ptr(),
                buf.len() as u32,
            )
        } as usize;

        if written == 0 {
            log("[SDK] query_data: host returned empty response");
            return None;
        }

        if written > buf.len() {
            buf_size = written;
            continue;
        }

        if written == buf.len() {
            log(&format!(
                "[SDK] query_data: response may have been truncated (filled {}/{} buffer)",
                written, buf_size
            ));
        }

        return Some(
            String::from_utf8_lossy(&buf[..written]).into_owned()
        );
    }

    log(&format!(
        "[SDK] query_data: exceeded max retries ({}) for buffer allocation",
        MAX_RETRIES
    ));
    None
}

/// Almacena un valor en el key-value store del host.
/// Versión silenciosa (compatibilidad hacia atrás).
pub fn kv_set_val(key: &str, value: &str) {
    let _ = kv_set_val_checked(key, value);
}

/// Almacena un valor en el key-value store del host.
/// Retorna Ok(()) si fue exitoso, o Error con el motivo del fallo.
pub fn kv_set_val_checked(key: &str, value: &str) -> Result<(), String> {
    if key.is_empty() {
        return Err("kv_set: key cannot be empty".to_string());
    }
    if value.len() > 1_048_576 {
        return Err(format!("kv_set: value too large ({} bytes, max 1MB)", value.len()));
    }
    unsafe {
        host_kv_set(key.as_ptr(), key.len() as u32, value.as_ptr(), value.len() as u32);
    }
    Ok(())
}

/// Lee un valor del key-value store del host.
///
/// Usa un buffer dinámico con reintento automático si el valor no cabe.
pub fn kv_get_val(key: &str) -> Option<String> {
    let mut buf_size = KV_BUF_SIZE;
    for _ in 0..MAX_RETRIES {
        let mut buf = vec![0u8; buf_size];
        let written = unsafe {
            host_kv_read(
                key.as_ptr(),
                key.len() as u32,
                buf.as_mut_ptr(),
                buf.len() as u32,
            )
        } as usize;

        if written == 0 {
            log(&format!("[SDK] kv_get_val: no value found for key '{}'", key));
            return None;
        }

        if written > buf.len() {
            buf_size = written;
            continue;
        }

        if written == buf.len() {
            log(&format!(
                "[SDK] kv_get_val: value for key '{}' may have been truncated (filled {}/{} buffer)",
                key, written, buf_size
            ));
        }

        return Some(
            String::from_utf8_lossy(&buf[..written]).into_owned()
        );
    }

    log(&format!(
        "[SDK] kv_get_val: exceeded max retries ({}) for key '{}'",
        MAX_RETRIES, key
    ));
    None
}

// ══════════════════════════════════════════════════════════════════════════
//  OAUTH FUNCTIONS
// ══════════════════════════════════════════════════════════════════════════

/// Inicia un flujo OAuth con el proveedor especificado.
/// Retorna la URL de autorización para redirigir al usuario.
pub fn oauth_start(provider: &str) -> Option<String> {
    let mut buf_size = INITIAL_BUF_SIZE;
    for _ in 0..MAX_RETRIES {
        let mut buf = vec![0u8; buf_size];
        let written = unsafe {
            host_oauth_start(
                provider.as_ptr(),
                provider.len() as u32,
                buf.as_mut_ptr(),
                buf.len() as u32,
            )
        } as usize;

        if written == 0 {
            log(&format!("[SDK] oauth_start: host returned empty response"));
            return None;
        }

        if written > buf.len() {
            buf_size = written;
            continue;
        }

        return Some(String::from_utf8_lossy(&buf[..written]).into_owned());
    }

    log(&format!(
        "[SDK] oauth_start: exceeded max retries ({})",
        MAX_RETRIES
    ));
    None
}

/// Procesa el callback de OAuth y retorna el token de acceso.
pub fn oauth_callback(callback_data: &str) -> Option<String> {
    let mut buf_size = INITIAL_BUF_SIZE;
    for _ in 0..MAX_RETRIES {
        let mut buf = vec![0u8; buf_size];
        let written = unsafe {
            host_oauth_callback(
                callback_data.as_ptr(),
                callback_data.len() as u32,
                buf.as_mut_ptr(),
                buf.len() as u32,
            )
        } as usize;

        if written == 0 {
            log(&format!("[SDK] oauth_callback: host returned empty response"));
            return None;
        }

        if written > buf.len() {
            buf_size = written;
            continue;
        }

        return Some(String::from_utf8_lossy(&buf[..written]).into_owned());
    }

    log(&format!(
        "[SDK] oauth_callback: exceeded max retries ({})",
        MAX_RETRIES
    ));
    None
}

// ══════════════════════════════════════════════════════════════════════════
//  SMS/PHONE FUNCTIONS
// ══════════════════════════════════════════════════════════════════════════

/// Envía un SMS a través del host.
/// Retorna Some(message_id) si fue exitoso, None si falló.
pub fn send_sms(to: &str, body: &str) -> Option<String> {
    let payload = serde_json::json!({
        "action": "send_sms",
        "to": to,
        "body": body,
    });
    
    match serde_json::to_string(&payload) {
        Ok(json) => {
            unsafe { host_publish_response(json.as_ptr(), json.len() as u32) };
            Some("sms_sent".to_string())
        }
        Err(e) => {
            log(&format!("[SDK] send_sms: error serializing: {}", e));
            None
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════
//  PHONE/CALL FUNCTIONS
// ══════════════════════════════════════════════════════════════════════════

/// Realiza una llamada telefónica a través del host.
/// Retorna Some(call_sid) si fue exitoso, None si falló.
pub fn make_call(to: &str, url: &str) -> Option<String> {
    let payload = serde_json::json!({
        "action": "make_call",
        "to": to,
        "url": url,
    });
    
    match serde_json::to_string(&payload) {
        Ok(json) => {
            unsafe { host_publish_response(json.as_ptr(), json.len() as u32) };
            Some("call_initiated".to_string())
        }
        Err(e) => {
            log(&format!("[SDK] make_call: error serializing: {}", e));
            None
        }
    }
}

/// Obtiene el estado de una llamada
pub fn get_call_status(call_sid: &str) -> Option<String> {
    let payload = serde_json::json!({
        "action": "get_call_status",
        "call_sid": call_sid,
    });
    
    match serde_json::to_string(&payload) {
        Ok(json) => {
            unsafe { host_publish_response(json.as_ptr(), json.len() as u32) };
            Some("status_unknown".to_string())
        }
        Err(e) => {
            log(&format!("[SDK] get_call_status: error serializing: {}", e));
            None
        }
    }
}

/// Obtiene el estado de un SMS enviado
pub fn get_sms_status(message_id: &str) -> Option<String> {
    let payload = serde_json::json!({
        "action": "get_sms_status",
        "message_id": message_id,
    });
    
    match serde_json::to_string(&payload) {
        Ok(json) => {
            unsafe { host_publish_response(json.as_ptr(), json.len() as u32) };
            Some("status_unknown".to_string())
        }
        Err(e) => {
            log(&format!("[SDK] get_sms_status: error serializing: {}", e));
            None
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════
//  CUSTOM DATA MODEL FUNCTIONS
// ══════════════════════════════════════════════════════════════════════════

/// Crea un modelo de datos personalizado para el plugin.
/// Retorna Some(model_id) si fue exitoso, None si falló.
pub fn create_data_model(model_name: &str, description: &str, schema: &str) -> Option<String> {
    let payload = serde_json::json!({
        "action": "create_data_model",
        "model_name": model_name,
        "description": description,
        "schema": schema,
    });
    
    match serde_json::to_string(&payload) {
        Ok(json) => {
            unsafe { host_publish_response(json.as_ptr(), json.len() as u32) };
            Some("model_created".to_string())
        }
        Err(e) => {
            log(&format!("[SDK] create_data_model: error serializing: {}", e));
            None
        }
    }
}

/// Lista los modelos de datos del plugin
pub fn list_data_models() -> Option<String> {
    let payload = serde_json::json!({
        "action": "list_data_models",
    });
    
    match serde_json::to_string(&payload) {
        Ok(json) => {
            unsafe { host_publish_response(json.as_ptr(), json.len() as u32) };
            Some("models_listed".to_string())
        }
        Err(e) => {
            log(&format!("[SDK] list_data_models: error serializing: {}", e));
            None
        }
    }
}

/// Crea un registro en un modelo de datos
pub fn create_data_record(model_name: &str, data: &str) -> Option<String> {
    let payload = serde_json::json!({
        "action": "create_data_record",
        "model_name": model_name,
        "data": data,
    });
    
    match serde_json::to_string(&payload) {
        Ok(json) => {
            unsafe { host_publish_response(json.as_ptr(), json.len() as u32) };
            Some("record_created".to_string())
        }
        Err(e) => {
            log(&format!("[SDK] create_data_record: error serializing: {}", e));
            None
        }
    }
}

/// Lista registros de un modelo de datos
pub fn list_data_records(model_name: &str, limit: i32) -> Option<String> {
    let payload = serde_json::json!({
        "action": "list_data_records",
        "model_name": model_name,
        "limit": limit,
    });
    
    match serde_json::to_string(&payload) {
        Ok(json) => {
            unsafe { host_publish_response(json.as_ptr(), json.len() as u32) };
            Some("records_listed".to_string())
        }
        Err(e) => {
            log(&format!("[SDK] list_data_records: error serializing: {}", e));
            None
        }
    }
}

/// Actualiza un registro en un modelo de datos
pub fn update_data_record(model_name: &str, record_id: &str, data: &str) -> Option<String> {
    let payload = serde_json::json!({
        "action": "update_data_record",
        "model_name": model_name,
        "record_id": record_id,
        "data": data,
    });
    
    match serde_json::to_string(&payload) {
        Ok(json) => {
            unsafe { host_publish_response(json.as_ptr(), json.len() as u32) };
            Some("record_updated".to_string())
        }
        Err(e) => {
            log(&format!("[SDK] update_data_record: error serializing: {}", e));
            None
        }
    }
}

/// Elimina un registro de un modelo de datos
pub fn delete_data_record(model_name: &str, record_id: &str) -> Option<String> {
    let payload = serde_json::json!({
        "action": "delete_data_record",
        "model_name": model_name,
        "record_id": record_id,
    });
    
    match serde_json::to_string(&payload) {
        Ok(json) => {
            unsafe { host_publish_response(json.as_ptr(), json.len() as u32) };
            Some("record_deleted".to_string())
        }
        Err(e) => {
            log(&format!("[SDK] delete_data_record: error serializing: {}", e));
            None
        }
    }
}

/// Cuenta registros de un modelo de datos
pub fn count_data_records(model_name: &str) -> Option<i32> {
    let payload = serde_json::json!({
        "action": "count_data_records",
        "model_name": model_name,
    });
    
    match serde_json::to_string(&payload) {
        Ok(json) => {
            unsafe { host_publish_response(json.as_ptr(), json.len() as u32) };
            Some(0) // Placeholder
        }
        Err(e) => {
            log(&format!("[SDK] count_data_records: error serializing: {}", e));
            None
        }
    }
}

/// Serializa una respuesta y la envía al host.
/// IMPORTANTE: Usa host_publish_response (no host_log) para que el backend lea la respuesta.
pub fn to_host_response<T: Serialize>(response: &T) {
    match serde_json::to_string(response) {
        Ok(json) => unsafe { host_publish_response(json.as_ptr(), json.len() as u32) },
        Err(e) => log(&format!("[SDK] Error serializing response: {}", e)),
    }
}

// ══════════════════════════════════════════════════════════════════════════
//  UI WIDGET FACTORY FUNCTIONS
//  ══════════════════════════════════════════════════════════════════════════
//  Uso: sdk::card("Título", [sdk::text("texto", "info")])
//       sdk::button("Click", "accion", "primary")
//       sdk::input("Email", "email", "tu@email.com")
//  ══════════════════════════════════════════════════════════════════════════

/// Crea un widget `Card` que puede contener hijos.
pub fn card(title: &str, children: Vec<UiWidget>) -> UiWidget {
    UiWidget::Card { title: title.to_string(), children, colspan: None }
}

/// Crea un widget `Text` con contenido y estilo.
pub fn text(content: &str, style: &str) -> UiWidget {
    UiWidget::Text { content: content.to_string(), style: style.to_string() }
}

/// Crea un widget `Button` con etiqueta, acción y variante visual.
pub fn button(label: &str, action: &str, variant: &str) -> UiWidget {
    UiWidget::Button { label: label.to_string(), action: action.to_string(), variant: variant.to_string() }
}

/// Crea un widget `Input` de texto.
pub fn input(label: &str, name: &str, placeholder: &str) -> UiWidget {
    UiWidget::Input { label: label.to_string(), name: name.to_string(), placeholder: placeholder.to_string(), value: String::new() }
}

/// Crea un widget `Textarea` multilínea.
pub fn textarea(label: &str, name: &str, placeholder: &str) -> UiWidget {
    UiWidget::Textarea { label: label.to_string(), name: name.to_string(), placeholder: placeholder.to_string(), value: String::new() }
}

/// Crea un widget `Select` (desplegable) con opciones.
pub fn select_widget(label: &str, name: &str, options: Vec<(&str, &str)>, value: &str) -> UiWidget {
    UiWidget::Select {
        label: label.to_string(),
        name: name.to_string(),
        options: options.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        value: value.to_string(),
    }
}

/// Crea un widget `Switch` (toggle) booleano.
pub fn switch_widget(label: &str, name: &str, value: bool) -> UiWidget {
    UiWidget::Switch { label: label.to_string(), name: name.to_string(), value }
}

/// Crea un widget `Badge` con contenido y variante visual.
pub fn badge(content: &str, variant: &str) -> UiWidget {
    UiWidget::Badge { content: content.to_string(), variant: variant.to_string() }
}

/// Crea un widget `Icon` con nombre y color.
pub fn icon(name: &str, color: &str) -> UiWidget {
    UiWidget::Icon { name: name.to_string(), color: color.to_string() }
}

/// Crea un widget `Divider` (línea separadora).
pub fn divider() -> UiWidget {
    UiWidget::Divider
}

/// Crea un widget `Modal` con título, hijos, tamaño y acción de cierre.
pub fn modal(title: &str, children: Vec<UiWidget>, size: &str, close_action: &str) -> UiWidget {
    UiWidget::Modal {
        title: title.to_string(),
        children,
        size: size.to_string(),
        close_action: close_action.to_string(),
    }
}

/// Crea un widget `Table` con headers y filas de datos.
pub fn table(headers: Vec<&str>, rows: Vec<Vec<&str>>) -> UiWidget {
    UiWidget::Table {
        headers: headers.into_iter().map(|h| h.to_string()).collect(),
        rows: rows.into_iter().map(|r| r.into_iter().map(|c| c.to_string()).collect()).collect(),
        caption: None,
    }
}

/// Crea un widget `Table` con caption (título de la tabla).
pub fn table_with_caption(headers: Vec<&str>, rows: Vec<Vec<&str>>, caption: &str) -> UiWidget {
    UiWidget::Table {
        headers: headers.into_iter().map(|h| h.to_string()).collect(),
        rows: rows.into_iter().map(|r| r.into_iter().map(|c| c.to_string()).collect()).collect(),
        caption: Some(caption.to_string()),
    }
}

/// Crea un widget `Chart` para visualización de datos.
/// Tipos soportados: "bar", "line", "pie", "gauge"
pub fn chart(title: &str, data: Vec<(&str, f64)>, chart_type: &str) -> UiWidget {
    UiWidget::Chart {
        title: title.to_string(),
        data: data.into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
        chart_type: chart_type.to_string(),
    }
}

/// Crea un widget `NumberInput` para entrada de números con validación.
pub fn number_input(label: &str, name: &str, placeholder: &str, value: &str) -> UiWidget {
    UiWidget::NumberInput {
        label: label.to_string(),
        name: name.to_string(),
        placeholder: placeholder.to_string(),
        value: value.to_string(),
        min: None,
        max: None,
        step: None,
    }
}

/// Crea un widget `NumberInput` con límites min/max/step.
pub fn number_input_with_limits(
    label: &str,
    name: &str,
    placeholder: &str,
    value: &str,
    min: f64,
    max: f64,
    step: f64,
) -> UiWidget {
    UiWidget::NumberInput {
        label: label.to_string(),
        name: name.to_string(),
        placeholder: placeholder.to_string(),
        value: value.to_string(),
        min: Some(min),
        max: Some(max),
        step: Some(step),
    }
}

/// Crea un widget `DateInput` para entrada de fechas.
pub fn date_input(label: &str, name: &str, placeholder: &str, value: &str) -> UiWidget {
    UiWidget::DateInput {
        label: label.to_string(),
        name: name.to_string(),
        placeholder: placeholder.to_string(),
        value: value.to_string(),
    }
}

/// Crea un widget `Calendar` con fecha seleccionada y eventos.
pub fn calendar(label: &str, name: &str, selected_date: &str, events: Vec<CalendarEvent>) -> UiWidget {
    UiWidget::Calendar {
        label: label.to_string(),
        name: name.to_string(),
        selected_date: selected_date.to_string(),
        events,
    }
}

/// Crea un evento de calendario.
pub fn calendar_event(date: &str, title: &str, color: &str) -> CalendarEvent {
    CalendarEvent {
        date: date.to_string(),
        title: title.to_string(),
        color: color.to_string(),
    }
}

/// Crea un widget `FileUpload` para subir archivos.
pub fn file_upload(label: &str, name: &str, accept: &str) -> UiWidget {
    UiWidget::FileUpload {
        label: label.to_string(),
        name: name.to_string(),
        accept: accept.to_string(),
        max_size: None,
    }
}

/// Crea un widget `FileUpload` con tamaño máximo.
pub fn file_upload_with_size(label: &str, name: &str, accept: &str, max_size: i32) -> UiWidget {
    UiWidget::FileUpload {
        label: label.to_string(),
        name: name.to_string(),
        accept: accept.to_string(),
        max_size: Some(max_size),
    }
}

/// Crea un widget `RichText` para editor de texto enriquecido.
pub fn rich_text(label: &str, name: &str, value: &str, placeholder: &str) -> UiWidget {
    UiWidget::RichText {
        label: label.to_string(),
        name: name.to_string(),
        value: value.to_string(),
        placeholder: placeholder.to_string(),
    }
}

// ══════════════════════════════════════════════════════════════════════════
//  NAVITEM BUILDER
//  ══════════════════════════════════════════════════════════════════════════
//  Uso: NavItem::new("dashboard", "Mi Plugin", "rocket-line")
//         .category("operaciones")
//         .priority(10)
//  ══════════════════════════════════════════════════════════════════════════

impl NavItem {
    /// Crea un nuevo `NavItem` con los campos obligatorios.
    pub fn new(page_id: &str, label: &str, icon: &str) -> Self {
        Self {
            page_id: page_id.to_string(),
            label: label.to_string(),
            icon: icon.to_string(),
            category: String::new(),
            priority: 0,
        }
    }

    /// Asigna la categoría de navegación.
    pub fn category(mut self, cat: &str) -> Self {
        self.category = cat.to_string();
        self
    }

    /// Asigna la prioridad de ordenamiento.
    pub fn priority(mut self, prio: i32) -> Self {
        self.priority = prio;
        self
    }
}

// ══════════════════════════════════════════════════════════════════════════
//  PLUGIN METADATA BUILDER
//  ══════════════════════════════════════════════════════════════════════════
//  Uso: sdk::metadata()
//         .nav_item(NavItem::new(...))
//         .name("Mi Plugin")
//         .version("1.0.0")
//         .author("RFJ Software")
//  ══════════════════════════════════════════════════════════════════════════

impl Default for PluginMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginMetadata {
    /// Crea un nuevo `PluginMetadata` con valores por defecto.
    pub fn new() -> Self {
        Self {
            host: "ezerdesk".to_string(),
            navigation: vec![],
            name: None,
            description: None,
            version: None,
            author: None,
        }
    }

    /// Agrega un elemento de navegación.
    pub fn nav_item(mut self, item: NavItem) -> Self {
        self.navigation.push(item);
        self
    }

    /// Asigna el nombre del plugin.
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Asigna la descripción del plugin.
    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    /// Asigna la versión semántica del plugin.
    pub fn version(mut self, ver: &str) -> Self {
        self.version = Some(ver.to_string());
        self
    }

    /// Asigna el autor del plugin.
    pub fn author(mut self, author: &str) -> Self {
        self.author = Some(author.to_string());
        self
    }
}

// ══════════════════════════════════════════════════════════════════════════
//  RESPONSE HELPERS
//  ══════════════════════════════════════════════════════════════════════════
//  sdk::respond(widgets)       → envía UI al host
//  sdk::respond_ok("msg")      → envía ActionResponse exitoso
//  sdk::respond_error("msg")   → envía ActionResponse con error
//  ══════════════════════════════════════════════════════════════════════════

/// Envía una respuesta con widgets de UI al host.
pub fn respond(widgets: Vec<UiWidget>) {
    to_host_response(&PluginResponse { success: true, ui_widgets: widgets })
}

/// Envía una respuesta de éxito simple al host.
pub fn respond_ok(message: &str) {
    to_host_response(&ActionResponse { success: true, response: message.to_string() })
}

/// Envía una respuesta de error simple al host.
pub fn respond_error(message: &str) {
    to_host_response(&ActionResponse { success: false, response: message.to_string() })
}

// ══════════════════════════════════════════════════════════════════════════
//  WIDGETS MACRO
//  ══════════════════════════════════════════════════════════════════════════
//  sdk::respond(sdk::widgets![
//      sdk::card("Título", [sdk::text("Hola", "info")]),
//  ]);
//  ══════════════════════════════════════════════════════════════════════════

/// Macro para construir un `Vec<UiWidget>` de forma concisa.
///
/// # Ejemplo
/// ```ignore
/// sdk::respond(sdk::widgets![
///     sdk::card("Título", vec![sdk::text("Hola", "info")]),
/// ]);
/// ```
#[macro_export]
macro_rules! widgets {
    ($($widget:expr),* $(,)?) => {
        vec![$($widget),*]
    };
}

// ══════════════════════════════════════════════════════════════════════════
//  PRELUDE
//  ══════════════════════════════════════════════════════════════════════════
//  use sdk::prelude::*;
//  ══════════════════════════════════════════════════════════════════════════

pub mod prelude {
    pub use super::{
        ActionResponse, NavItem, PluginEvent, PluginMetadata, PluginResponse, UiWidget,
    };
    pub use super::{
        badge, button, calendar, calendar_event, card, chart, date_input, divider, file_upload,
        file_upload_with_size, icon, input, modal, number_input, number_input_with_limits,
        respond, respond_error, respond_ok, rich_text, select_widget, switch_widget, table,
        table_with_caption, text, textarea,
    };
    pub use super::{http_request, kv_get_val, kv_set_val, kv_set_val_checked, log, oauth_start, oauth_callback, query_data, send_sms, get_sms_status, make_call, get_call_status, to_host_response, create_data_model, list_data_models, create_data_record, list_data_records, update_data_record, delete_data_record, count_data_records};
    pub use super::query::{self, TicketSummary, AgentSummary, DepartmentSummary, 
        ChatSessionSummary, ChatMessageSummary, WorkflowSummary, SlaPolicySummary, AnalyticsSummary};
}

/// Asigna memoria en el heap de WASM y devuelve un puntero.
///
/// Usado internamente por la macro `#[sdk::main]` para la ABI de
/// `alloc`/`deallocate` con el host.
pub fn allocate(size: usize) -> *mut u8 {
    let actual = if size == 0 { 1 } else { size };
    let mut buf = Vec::with_capacity(actual);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// Libera memoria previamente asignada con [`allocate`].
///
/// # Safety
/// `ptr` debe ser un puntero válido devuelto por [`allocate`] y `size` debe
/// coincidir con el tamaño solicitado originalmente.
pub unsafe fn deallocate(ptr: *mut u8, size: usize) {
    unsafe { let _ = Vec::from_raw_parts(ptr, 0, size); }
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
        unsafe { deallocate(ptr, 1) };
    }

    #[test]
    fn test_allocate_and_deallocate() {
        let ptr = allocate(64);
        assert!(!ptr.is_null());
        unsafe {
            std::ptr::write_bytes(ptr, 0xAB, 64);
            deallocate(ptr, 64);
        }
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
