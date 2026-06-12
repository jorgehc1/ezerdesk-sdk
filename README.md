# Ezerdesk SDK 💠

The official Rust SDK for building WebAssembly plugins for the **Ezerdesk** helpdesk system.

## Overview

`ezerdesk-sdk` provides the necessary types, macros, and host bindings to create dynamic, memory-safe plugins that run inside the Ezerdesk Wasm engine. It allows you to define UI components, handle system events, manage persistent state, create custom data models, and integrate with external services via OAuth and SMS.

## Key Features

### 🎨 UI Components (16 widgets)
- **Cards, Text, Buttons** - Basic layout and interaction
- **Inputs, Textareas, Selects, Switches** - Form elements
- **Badges, Icons, Dividers** - Visual indicators
- **Modals** - Dialog overlays
- **Tables** - Structured data display with headers and rows
- **Charts** - Data visualization (bar, line, pie)
- **NumberInput** - Numeric input with min/max/step validation
- **DateInput** - Date picker input

### 📊 Data Access (40+ query types)
- **tickets** - Query helpdesk tickets with status filters
- **agents** - List users and agents
- **departments** - List organizational departments
- **chat_sessions** - Query live chat sessions with sentiment
- **chat_messages** - Query chat messages by session
- **workflows** - List automation workflows
- **sla_policies** - Query SLA policies
- **analytics** - System-wide metrics (tickets, agents, trends)
- **Plus 30+ more query types** for all system entities

### 🔐 Security & Auth
- **OAuth Support** - Connect to external services (Google, Slack, GitHub)
- **Sandboxed Execution** - WASM isolation with fuel/memory limits
- **SSRF Protection** - Blocked private IPs and metadata endpoints
- **KV Store** - Persistent key-value storage per plugin

### ⏰ Scheduling
- **Cron Jobs** - Register periodic tasks (hourly, daily, weekly)
- **Event-Driven** - React to 24+ system events

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
ezerdesk-sdk = "0.1.3"
serde = { version = "1.0", features = ["derive"] }
```

### Basic Plugin

```rust
use ezerdesk_sdk as sdk;
use sdk::prelude::*;

#[sdk::main]
fn main(event: PluginEvent) -> i32 {
    match event {
        PluginEvent::GetMetadata => {
            let meta = PluginMetadata::new()
                .nav_item(NavItem::new("dashboard", "Mi Plugin", "rocket-line")
                    .category("operaciones")
                    .priority(10))
                .name("Mi Plugin")
                .description("Descripción del plugin")
                .version(env!("CARGO_PKG_VERSION"));
            sdk::to_host_response(&meta);
        }
        PluginEvent::PageRequest { page_id } => {
            if page_id == "dashboard" {
                sdk::respond(sdk::widgets![
                    sdk::card("Panel Principal", vec![
                        sdk::text("Bienvenido a tu plugin.", "info")
                    ])
                ]);
            }
        }
        _ => {}
    }
    0
}
```

### Advanced: Tables & Charts

```rust
// Tabla de datos
sdk::respond(sdk::widgets![
    sdk::table(
        vec!["ID", "Asunto", "Estado", "Prioridad"],
        vec![
            vec!["123", "Error login", "Abierto", "Alta"],
            vec!["124", "Solicitud features", "En progreso", "Media"],
        ]
    )
]);

// Gráfico de barras
sdk::respond(sdk::widgets![
    sdk::chart("Tickets por día", vec![
        ("Lun", 12.0), ("Mar", 19.0), ("Mié", 8.0),
        ("Jue", 15.0), ("Vie", 22.0),
    ], "bar")
]);
```

### Advanced: Data Queries

```rust
// Consultar tickets con filtros
let tickets = sdk::query::tickets()
    .by_status("Abierto")
    .limit(10)
    .all()?;

// Consultar sesiones de chat
let sessions = sdk::query::chat_sessions().all()?;

// Consultar analytics del sistema
let analytics = sdk::query::analytics().get()?;
println!("Tickets totales: {}", analytics.total_tickets);
println!("Agentes activos: {}", analytics.agentes_activos);
```

### Advanced: OAuth Integration

```rust
// Iniciar flujo OAuth con Google
if let Some(auth_url) = sdk::oauth_start("google") {
    // Redirigir al usuario a auth_url
}

// Procesar callback de OAuth
if let Some(token_data) = sdk::oauth_callback("code=abc123&state=xyz") {
    sdk::kv_set_val("google_token", &token_data);
}
```

### Advanced: HTTP Requests

```rust
let req = HttpRequest {
    method: "GET".to_string(),
    url: "https://api.ipify.org?format=json".to_string(),
    body: "".to_string(),
    headers: vec![],
};

if let Some(res) = sdk::http_request(&req) {
    sdk::kv_set_val("last_ip", &res.body);
}
```

### Advanced: Custom Data Models

Los modelos de datos custom permiten a los plugins crear y gestionar sus propias tablas de datos.

```rust
// Crear modelo "inventario"
sdk::create_data_model(
    "inventario",
    "Gestión de inventario de productos",
    r#"{"fields": [
        {"name": "producto", "type": "string", "required": true},
        {"name": "cantidad", "type": "number", "min": 0},
        {"name": "categoria", "type": "string"}
    ]}"#
);

// Crear registro
sdk::create_data_record("inventario", 
    r#"{"producto": "Laptop", "cantidad": 10, "categoria": "Electrónica"}"#
);

// Listar registros
if let Some(records) = sdk::list_data_records("inventario", 100) {
    sdk::log(&format!("Registros: {}", records));
}

// Actualizar registro
sdk::update_data_record("inventario", "record-123", 
    r#"{"cantidad": 5}"#
);

// Eliminar registro
sdk::delete_data_record("inventario", "record-123");

// Contar registros
if let Some(count) = sdk::count_data_records("inventario") {
    sdk::log(&format!("Total: {}", count));
}
```

### Advanced: Persistent State

```rust
// Guardar valor
sdk::kv_set_val("counter", "42");

// Leer valor
if let Some(value) = sdk::kv_get_val("counter") {
    sdk::log(&format!("Counter: {}", value));
}
```

### Advanced: Cron Scheduling

```rust
// Registrar tarea diaria (se ejecuta cada 86400 segundos)
// El plugin debe manejar el evento "cron_tick"
PluginEvent::GetMetadata => {
    let meta = PluginMetadata::new()
        .nav_item(NavItem::new("reports", "Reportes", "file-list-line")
            .category("operaciones")
            .priority(20))
        .name("Reportes Diarios")
        .cron("86400")  // Cada 24 horas
        .version("1.0.0");
    sdk::to_host_response(&meta);
}
```

## Available Events

| Event | Description |
|-------|-------------|
| `GetMetadata` | Plugin metadata and navigation |
| `PageRequest` | User clicked plugin page |
| `GetUiFragments` | UI injection at specific locations |
| `PluginAction` | User triggered an action |
| `TicketCreated` | New ticket created |
| `TicketUpdated` | Ticket updated |
| `TicketStatusChanged` | Ticket status changed |
| `CommentAdded` | Comment added to ticket |
| `SessionCreated` | Chat session started |
| `ChatMessageCreated` | New chat message |
| `HandoverRequested` | Agent handover requested |
| `AgentCreated` / `AgentDeleted` | Agent lifecycle |
| `WorkflowTriggered` | Workflow executed |
| `SlaBreachDetected` | SLA breach detected |
| `PluginUpdated` / `PluginDeleted` | Plugin lifecycle |
| `CronTick` | Scheduled task execution |

## Widget Types

| Widget | Properties | Use Case |
|--------|-----------|----------|
| `Card` | title, children, colspan | Container |
| `Text` | content, style | Styled paragraph |
| `Button` | label, action, variant | Click handler |
| `Input` | label, name, placeholder, value | Text input |
| `Textarea` | label, name, placeholder, value | Multi-line input |
| `Select` | label, name, options, value | Dropdown |
| `Switch` | label, name, value | Toggle switch |
| `Badge` | content, variant | Status indicator |
| `Icon` | name, color | Icon display |
| `Divider` | - | Separator |
| `Modal` | title, children, size, close_action | Dialog |
| `Table` | headers, rows, caption | Data table |
| `Chart` | title, data, chart_type | Data visualization |

## Query Types

| Type | Returns | Filters |
|------|---------|---------|
| `tickets` | id, asunto, estado, prioridad, creado_en | `by_status()` |
| `agents` | id, nombres, apellidos, correo | - |
| `departments` | id, nombre | - |
| `chat_sessions` | id, estado, sentimiento, etiqueta, agente | - |
| `chat_messages` | id, id_sesion, tipo_emisor, contenido, creado_en | `by_session()` |
| `workflows` | id, nombre, activo, disparador, datos_canvas | - |
| `sla_policies` | id, departamento, prioridad, tiempos | - |
| `analytics` | total_tickets, abiertos, cerrados, ultima_semana, agentes_activos | - |

## Host Functions

| Function | Description |
|----------|-------------|
| `host_publish_response` | Send JSON response to host |
| `host_log` | Send log message (separate from response) |
| `host_kv_set` | Write to KV store |
| `host_kv_read` | Read from KV store |
| `host_http_request` | Make HTTP request through host proxy |
| `host_query` | Execute data query against backend |
| `host_oauth_start` | Start OAuth flow with provider |
| `host_oauth_callback` | Process OAuth callback |

## Security

Plugins run in a sandboxed environment with:
- **Fuel metering**: 1M instruction limit per execution
- **Memory limits**: 50MB max per plugin
- **Import validation**: Only whitelisted host functions allowed
- **SSRF protection**: Private IPs and metadata endpoints blocked
- **Execution timeout**: 5-second limit per event (10s for cron)

## License

Apache-2.0
