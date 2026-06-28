use serde::{Deserialize, Serialize};

// ══════════════════════════════════════════════════════════════════════════
//  TIPOS DE RESPUESTA
//  ══════════════════════════════════════════════════════════════════════════

/// Resultado de una consulta al sistema
#[derive(Debug)]
pub enum QueryError {
    Network,
    Parse(String),
    Backend(String),
}

/// Ticket resumido para listados (viene de data querying)
#[derive(Deserialize, Debug, Clone)]
pub struct TicketSummary {
    pub id: String,
    pub asunto: String,
    pub estado: String,
    pub prioridad: String,
    pub creado_en: String,
}

/// Agente resumido para listados
#[derive(Deserialize, Debug, Clone)]
pub struct AgentSummary {
    pub id: String,
    pub nombres: String,
    pub apellidos: String,
    pub correo: String,
}

/// Departamento resumido para listados
#[derive(Deserialize, Debug, Clone)]
pub struct DepartmentSummary {
    pub id: String,
    pub nombre: String,
}

/// Sesión de chat resumida
#[derive(Deserialize, Debug, Clone)]
pub struct ChatSessionSummary {
    pub id: String,
    pub estado: String,
    pub sentimiento: f64,
    pub etiqueta_sentimiento: String,
    pub agente_asignado: String,
}

/// Mensaje de chat resumido
#[derive(Deserialize, Debug, Clone)]
pub struct ChatMessageSummary {
    pub id: String,
    pub id_sesion: String,
    pub tipo_emisor: String,
    pub contenido: String,
    pub creado_en: String,
}

/// Workflow resumido
#[derive(Deserialize, Debug, Clone)]
pub struct WorkflowSummary {
    pub id: String,
    pub nombre: String,
    pub activo: bool,
    pub disparador: String,
    pub datos_canvas: String,
}

/// Política SLA resumida
#[derive(Deserialize, Debug, Clone)]
pub struct SlaPolicySummary {
    pub id: String,
    pub id_departamento: String,
    pub id_prioridad: String,
    pub tiempo_respuesta_minutos: i32,
    pub tiempo_resolucion_minutos: i32,
}

/// Analytics del sistema
#[derive(Deserialize, Debug, Clone)]
pub struct AnalyticsSummary {
    pub total_tickets: i32,
    pub tickets_abiertos: i32,
    pub tickets_cerrados: i32,
    pub tickets_ultima_semana: i32,
    pub agentes_activos: i32,
}

/// Log de auditoría
#[derive(Deserialize, Debug, Clone)]
pub struct AuditLogSummary {
    pub id: String,
    pub recurso_tipo: String,
    pub recurso_id: String,
    pub accion: String,
    pub creado_en: String,
}

/// Gift card
#[derive(Deserialize, Debug, Clone)]
pub struct GiftCardSummary {
    pub codigo: String,
    pub monto: i32,
    pub estado: String,
    pub creado_en: String,
}

/// Notificación
#[derive(Deserialize, Debug, Clone)]
pub struct NotificationSummary {
    pub id: String,
    pub titulo: String,
    pub mensaje: String,
    pub leido: bool,
    pub creado_en: String,
}

/// Integración
#[derive(Deserialize, Debug, Clone)]
pub struct IntegrationSummary {
    pub id: String,
    pub provider: String,
    pub activo: bool,
    pub creado_en: String,
}

/// Feature flag
#[derive(Deserialize, Debug, Clone)]
pub struct FeatureFlagSummary {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub activo: bool,
}

// ══════════════════════════════════════════════════════════════════════════
//  WRAPPER DE RESPUESTA DEL BACKEND
//  ══════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
struct DataWrapper<T> {
    data: Vec<T>,
}

fn parse_response<T: for<'a> Deserialize<'a>>(json: &str) -> Result<Vec<T>, QueryError> {
    match serde_json::from_str::<DataWrapper<T>>(json) {
        Ok(w) => Ok(w.data),
        Err(_) => match serde_json::from_str::<Vec<T>>(json) {
            Ok(v) => Ok(v),
            Err(e) => Err(QueryError::Parse(format!(
                "Error decodificando respuesta: {}",
                e
            ))),
        },
    }
}

// ══════════════════════════════════════════════════════════════════════════
//  SNMP DEVICES QUERY
// ══════════════════════════════════════════════════════════════════════════

/// Dispositivo SNMP resumido para listados (viene de data querying).
#[derive(Deserialize, Debug, Clone)]
pub struct SnmpDeviceSummary {
    pub id: String,
    pub nombre: String,
    pub host: String,
    pub puerto: u16,
    pub comunidad: String,
    pub version: String,
    pub activo: bool,
    pub ultimo_ok_en: String,
}

/// Constructor para consultas de dispositivos SNMP
pub fn snmp_devices() -> SimpleQuery<SnmpDeviceSummary> {
    SimpleQuery::new("snmp_devices")
}

// ══════════════════════════════════════════════════════════════════════════
//  QUERY BUILDERS
//  ══════════════════════════════════════════════════════════════════════════

/// Constructor para consultas de tickets
pub fn tickets() -> TicketQuery {
    TicketQuery::new()
}

/// Constructor para consultas de agentes
pub fn agents() -> SimpleQuery<AgentSummary> {
    SimpleQuery::new("agents")
}

/// Constructor para consultas de departamentos
pub fn departments() -> SimpleQuery<DepartmentSummary> {
    SimpleQuery::new("departments")
}

/// Constructor para consultas de sesiones de chat
pub fn chat_sessions() -> SimpleQuery<ChatSessionSummary> {
    SimpleQuery::new("chat_sessions")
}

/// Constructor para consultas de mensajes de chat
pub fn chat_messages() -> ChatMessageQuery {
    ChatMessageQuery::new()
}

/// Constructor para consultas de workflows
pub fn workflows() -> SimpleQuery<WorkflowSummary> {
    SimpleQuery::new("workflows")
}

/// Constructor para consultas de políticas SLA
pub fn sla_policies() -> SimpleQuery<SlaPolicySummary> {
    SimpleQuery::new("sla_policies")
}

/// Constructor para analytics del sistema
pub fn analytics() -> AnalyticsQuery {
    AnalyticsQuery::new()
}

/// Constructor para logs de auditoría
pub fn audit_logs() -> SimpleQuery<AuditLogSummary> {
    SimpleQuery::new("audit_logs")
}

/// Constructor para gift cards
pub fn gift_cards() -> SimpleQuery<GiftCardSummary> {
    SimpleQuery::new("gift_cards")
}

/// Constructor para notificaciones
pub fn notifications() -> SimpleQuery<NotificationSummary> {
    SimpleQuery::new("notifications")
}

/// Constructor para integraciones
pub fn integrations() -> SimpleQuery<IntegrationSummary> {
    SimpleQuery::new("integrations")
}

/// Constructor para feature flags
pub fn feature_flags() -> SimpleQuery<FeatureFlagSummary> {
    SimpleQuery::new("feature_flags")
}

// ══════════════════════════════════════════════════════════════════════════
//  PAYLOAD STRUCTS (serialización segura mediante serde)
//  ══════════════════════════════════════════════════════════════════════════

#[derive(Serialize)]
struct TicketQueryPayload {
    #[serde(rename = "type")]
    query_type: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<TicketFilters>,
}

#[derive(Serialize)]
struct TicketFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_to: Option<String>,
}

#[derive(Serialize)]
struct SimpleQueryPayload {
    #[serde(rename = "type")]
    query_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

#[derive(Serialize)]
struct ChatMessageQueryPayload {
    #[serde(rename = "type")]
    query_type: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<ChatMessageFilters>,
}

#[derive(Serialize)]
struct ChatMessageFilters {
    session_id: String,
}

#[derive(Serialize)]
struct AnalyticsQueryPayload {
    #[serde(rename = "type")]
    query_type: &'static str,
}

// ══════════════════════════════════════════════════════════════════════════
//  TICKET QUERY (con filtros)
//  ══════════════════════════════════════════════════════════════════════════

pub struct TicketQuery {
    limit: Option<u32>,
    status: Option<String>,
    priority: Option<String>,
    assignee: Option<String>,
    search: Option<String>,
    date_from: Option<String>,
    date_to: Option<String>,
}

impl Default for TicketQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl TicketQuery {
    pub fn new() -> Self {
        Self {
            limit: None,
            status: None,
            priority: None,
            assignee: None,
            search: None,
            date_from: None,
            date_to: None,
        }
    }

    /// Limitar la cantidad de resultados
    pub fn limit(mut self, n: u32) -> Self {
        self.limit = Some(n);
        self
    }

    /// Filtrar por estado
    pub fn by_status(mut self, status: &str) -> Self {
        self.status = Some(status.to_string());
        self
    }

    /// Filtrar por prioridad
    pub fn by_priority(mut self, priority: &str) -> Self {
        self.priority = Some(priority.to_string());
        self
    }

    /// Filtrar por agente asignado
    pub fn by_assignee(mut self, assignee_id: &str) -> Self {
        self.assignee = Some(assignee_id.to_string());
        self
    }

    /// Buscar en asunto y descripción
    pub fn search(mut self, query: &str) -> Self {
        self.search = Some(query.to_string());
        self
    }

    /// Filtrar por fecha de creación desde
    pub fn date_from(mut self, date: &str) -> Self {
        self.date_from = Some(date.to_string());
        self
    }

    /// Filtrar por fecha de creación hasta
    pub fn date_to(mut self, date: &str) -> Self {
        self.date_to = Some(date.to_string());
        self
    }

    /// Ejecutar la consulta y obtener resultados
    pub fn all(&self) -> Result<Vec<TicketSummary>, QueryError> {
        let filters = TicketFilters {
            status: self.status.clone(),
            priority: self.priority.clone(),
            assignee: self.assignee.clone(),
            search: self.search.clone(),
            date_from: self.date_from.clone(),
            date_to: self.date_to.clone(),
        };

        let payload = TicketQueryPayload {
            query_type: "tickets",
            limit: self.limit,
            filters: Some(filters),
        };

        let body = match serde_json::to_string(&payload) {
            Ok(j) => j,
            Err(e) => {
                return Err(QueryError::Parse(format!("Error serializing query: {}", e)));
            }
        };

        match crate::query_data(&body) {
            Some(json) => parse_response::<TicketSummary>(&json),
            None => Err(QueryError::Network),
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════
//  SIMPLE QUERY (sin filtros)
//  ══════════════════════════════════════════════════════════════════════════

pub struct SimpleQuery<T> {
    query_type: String,
    limit: Option<u32>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> SimpleQuery<T> {
    pub fn new(query_type: &str) -> Self {
        Self {
            query_type: query_type.to_string(),
            limit: None,
            _marker: std::marker::PhantomData,
        }
    }

    /// Ejecutar la consulta y obtener resultados
    pub fn all(&self) -> Result<Vec<T>, QueryError>
    where
        T: for<'a> Deserialize<'a>,
    {
        let payload = SimpleQueryPayload {
            query_type: self.query_type.clone(),
            limit: self.limit,
        };

        let body = match serde_json::to_string(&payload) {
            Ok(j) => j,
            Err(e) => {
                return Err(QueryError::Parse(format!("Error serializing query: {}", e)));
            }
        };

        match crate::query_data(&body) {
            Some(json) => parse_response::<T>(&json),
            None => Err(QueryError::Network),
        }
    }

    /// Limitar la cantidad de resultados
    pub fn limit(mut self, n: u32) -> Self {
        self.limit = Some(n);
        self
    }
}

// ══════════════════════════════════════════════════════════════════════════
//  CHAT MESSAGE QUERY (con filtros)
// ══════════════════════════════════════════════════════════════════════════

pub struct ChatMessageQuery {
    limit: Option<u32>,
    session_id: Option<String>,
}

impl ChatMessageQuery {
    pub fn new() -> Self {
        Self {
            limit: None,
            session_id: None,
        }
    }

    pub fn limit(mut self, n: u32) -> Self {
        self.limit = Some(n);
        self
    }

    pub fn by_session(mut self, session_id: &str) -> Self {
        self.session_id = Some(session_id.to_string());
        self
    }

    pub fn all(&self) -> Result<Vec<ChatMessageSummary>, QueryError> {
        let payload = ChatMessageQueryPayload {
            query_type: "chat_messages",
            limit: self.limit,
            filters: self.session_id.as_ref().map(|s| ChatMessageFilters { session_id: s.clone() }),
        };

        let body = match serde_json::to_string(&payload) {
            Ok(j) => j,
            Err(e) => {
                return Err(QueryError::Parse(format!("Error serializing query: {}", e)));
            }
        };

        match crate::query_data(&body) {
            Some(json) => parse_response::<ChatMessageSummary>(&json),
            None => Err(QueryError::Network),
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════
//  ANALYTICS QUERY
// ══════════════════════════════════════════════════════════════════════════

pub struct AnalyticsQuery;

impl AnalyticsQuery {
    pub fn new() -> Self {
        Self
    }

    pub fn get(&self) -> Result<AnalyticsSummary, QueryError> {
        let payload = AnalyticsQueryPayload {
            query_type: "analytics",
        };

        let body = match serde_json::to_string(&payload) {
            Ok(j) => j,
            Err(e) => {
                return Err(QueryError::Parse(format!("Error serializing query: {}", e)));
            }
        };

        match crate::query_data(&body) {
            Some(json) => {
                match serde_json::from_str::<AnalyticsSummary>(&json) {
                    Ok(summary) => Ok(summary),
                    Err(e) => Err(QueryError::Parse(format!("Error parsing analytics: {}", e))),
                }
            }
            None => Err(QueryError::Network),
        }
    }
}
