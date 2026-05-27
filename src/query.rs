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

// ══════════════════════════════════════════════════════════════════════════
//  WRAPPER DE RESPUESTA DEL BACKEND
//  ══════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
struct DataWrapper<T> {
    data: Vec<T>,
}

fn parse_response<T: for<'a> Deserialize<'a>>(json: &str) -> Result<Vec<T>, QueryError> {
    // Intentar con wrapper { data: [...] }
    match serde_json::from_str::<DataWrapper<T>>(json) {
        Ok(w) => Ok(w.data),
        Err(_) => {
            // Intentar array directo
            match serde_json::from_str::<Vec<T>>(json) {
                Ok(v) => Ok(v),
                Err(e) => Err(QueryError::Parse(format!(
                    "Error decodificando respuesta: {}",
                    e
                ))),
            }
        }
    }
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
    status: String,
}

#[derive(Serialize)]
struct SimpleQueryPayload {
    #[serde(rename = "type")]
    query_type: String,
}

// ══════════════════════════════════════════════════════════════════════════
//  TICKET QUERY (con filtros)
//  ══════════════════════════════════════════════════════════════════════════

pub struct TicketQuery {
    limit: Option<u32>,
    status: Option<String>,
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

    /// Ejecutar la consulta y obtener resultados
    pub fn all(&self) -> Result<Vec<TicketSummary>, QueryError> {
        let payload = TicketQueryPayload {
            query_type: "tickets",
            limit: self.limit,
            filters: self.status.as_ref().map(|s| TicketFilters { status: s.clone() }),
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
    _marker: std::marker::PhantomData<T>,
}

impl<T> SimpleQuery<T> {
    pub fn new(query_type: &str) -> Self {
        Self {
            query_type: query_type.to_string(),
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
}
