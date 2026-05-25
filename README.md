# Ezerdesk SDK 💠

The official Rust SDK for building WebAssembly plugins for the **Ezerdesk** helpdesk system.

## Overview

`ezerdesk-sdk` provides the necessary types, macros, and host bindings to create dynamic, memory-safe plugins that run inside the Ezerdesk Wasm engine. It allows you to define UI components, handle system events, and manage persistent state within a multi-tenant environment.

## Key Features

- **Declarative UI**: Build interfaces using structured widgets (Cards, Inputs, Buttons, Modals, etc.).
- **HTTP Client**: Perform secure network requests through the host's proxy.
- **Event-Driven**: React to system events like `TicketCreated` or `PluginUpdated`.
- **KV Store**: Access a persistent key-value store provided by the host.
- **Multi-Host Ready**: Support for multiple host applications via compatibility metadata.
- **Macros**: Simplified entry point and serialization management.

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
ezerdesk-sdk = "0.1.4"
serde = { version = "1.0", features = ["derive"] }
```

### Advanced Usage (Modal & HTTP)

```rust
use ezerdesk_sdk as sdk;
use sdk::{UiWidget, PluginEvent, PluginResponse, HttpRequest};

#[sdk::main]
fn main(event: PluginEvent) -> i32 {
    match event {
        PluginEvent::PluginAction { action, .. } => {
            if action == "test_http" {
                let req = HttpRequest {
                    method: "GET".to_string(),
                    url: "https://api.ipify.org?format=json".to_string(),
                    body: "".to_string(),
                    headers: vec![],
                };

                if let Some(res) = sdk::http_request(&req) {
                    sdk::kv_set_val("last_ip", &res.body);
                }
            }
        }
        PluginEvent::GetUiFragments { location } => {
            if location == "plugin_settings" {
                let widgets = vec![
                    UiWidget::Modal {
                        title: "System Tools".to_string(),
                        size: "lg".to_string(),
                        close_action: "close_modal".to_string(),
                        children: vec![
                            UiWidget::Text { content: "Request tools:".to_string(), style: "default".to_string() },
                            UiWidget::Button { label: "Check IP".to_string(), action: "test_http".to_string(), variant: "primary".to_string() }
                        ]
                    }
                ];
                sdk::to_host_response(&PluginResponse { success: true, ui_widgets: widgets });
            }
        }
        _ => {}
    }
    0
}
```

## Security

Plugins run in a sandboxed environment with strict resource limits. Direct access to the filesystem or network is prohibited; all external interactions must use the provided SDK bridges which are audited by the host.

## License

Apache-2.0