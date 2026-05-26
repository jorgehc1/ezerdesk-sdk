use serde::Deserialize;

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
//  TICKET QUERY (con filtros)
//  ══════════════════════════════════════════════════════════════════════════

pub struct TicketQuery {
    limit: Option<u32>,
    status: Option<String>,
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
        let mut body = r#"{"type":"tickets""#.to_string();

        if let Some(ref s) = self.status {
            body.push_str(&format!(",\"filters\":{{\"status\":\"{}\"}}", s));
        }

        body.push_str("}");

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
        let body = format!("{{\"type\":\"{}\"}}", self.query_type);

        match crate::query_data(&body) {
            Some(json) => parse_response::<T>(&json),
            None => Err(QueryError::Network),
        }
    }
}
