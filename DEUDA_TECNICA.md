# Reporte de Deuda Técnica: `ezerdesk-sdk`

> **Nota del Analista:** Aunque solicitaste el análisis asumiendo un contexto en el lenguaje Gleam, he detectado que el proyecto `ezerdesk-sdk` está desarrollado completamente en **Rust** (con soporte de WebAssembly). El siguiente análisis aborda específicamente las áreas de mejora, vulnerabilidades y deuda técnica encontradas en el código fuente (principalmente `lib.rs` y `query.rs`) desde la perspectiva de un ingeniero de software experto.

Este documento está estructurado para que otra IA o el equipo de ingeniería pueda tomarlo y procesarlo como un backlog de refactorización.

## 1. Manejo de Memoria y Límites de Buffers Estáticos (Severidad: Alta)
- **Problema:** En funciones clave como `http_request`, `query_data` y `kv_get_val` (en `lib.rs`), se están utilizando arreglos estáticos locales como buffers (`[0u8; 65536]` y `[0u8; 16384]`).
- **Impacto:** Si la respuesta del backend supera estos límites de tamaño (ej. una consulta de tickets muy larga o una llamada HTTP con payload pesado), la respuesta será truncada o la función devolverá `None` (silenciosamente fallando).
- **Acción a tomar:** 
  - Refactorizar las llamadas FFI para soportar tamaños dinámicos de respuesta (ej. pasando la longitud requerida de antemano o usando memoria compartida administrada dinámicamente).

## 2. Construcción de JSON por Concatenación de Strings (Severidad: Media)
- **Problema:** En `query.rs`, específicamente en `TicketQuery::all` y `SimpleQuery::all`, el body de las peticiones JSON se construye utilizando interpolación de strings crudos: `format!("{{\"type\":\"{}\"}}", ...)` y `.push_str(...)`.
- **Impacto:** Esto es altamente propenso a errores de formato, vulnerabilidades de inyección y hace que el código sea frágil ante cambios. 
- **Adicional:** ¡El parámetro `limit` configurado en `TicketQuery` no se está inyectando en la consulta real! La variable existe pero se ignora al momento de construir el payload JSON.
- **Acción a tomar:**
  - Definir estructuras Rust (Structs) para las solicitudes y utilizar `serde_json::to_string` para serializarlas y garantizar el formato JSON correcto.

## 3. Manejo de Errores Silenciosos (Severidad: Media)
- **Problema:** A lo largo del SDK, múltiples errores resultan en un retorno pasivo de `None` sin registro (log) de lo ocurrido. Por ejemplo, en `http_request` (`serde_json::to_string(req)` falla y devuelve `None` en la línea 315).
- **Impacto:** Dificulta el debugging (rastreo de errores) para los desarrolladores de plugins cuando algo sale mal.
- **Acción a tomar:**
  - Implementar el retorno de un `Result<T, E>` en las funciones expuestas o usar `log(...)` antes de devolver `None` para informar el motivo del fallo.

## 4. Dependencia de Ruta Local (Severidad: Baja)
- **Problema:** En `Cargo.toml`, el macro crate se referencia localmente: `ezerdesk-sdk-macros = { version = "0.1.3", path = "../ezerdesk-sdk-macros" }`.
- **Impacto:** Si este SDK está planeado para ser publicado en `crates.io`, fallará.
- **Acción a tomar:**
  - Definir un workspace explícito y asegurar que durante el publish la dependencia esté referenciada por versión o mantenerlo documentado si es para uso exclusivamente interno.

## 5. Falta de Documentación de API (Severidad: Baja)
- **Problema:** Aunque la estructura está bien modularizada y contiene comentarios de separación de bloques, faltan *Rustdocs* (`///`) descriptivos en la gran mayoría de structs (`PluginResponse`, `UiWidget`, constructores, etc.).
- **Impacto:** La experiencia del desarrollador (DX) para los consumidores del SDK será pobre al no contar con autocompletado documentado en su IDE.
- **Acción a tomar:**
  - Generar comentarios de documentación `///` para toda la API pública exportada, especialmente explicando los argumentos de los constructores de Widgets de UI.

## 6. Eficiencia en Deserialización (Optimización)
- **Problema:** En el manejo de respuestas del *host* en `lib.rs`, se está convirtiendo primero a un `String` y luego a la estructura usando `String::from_utf8_lossy(&buf[...]).to_string()` seguido de `serde_json::from_str`.
- **Impacto:** Provoca copias y allocations de memoria innecesarias en un entorno donde WASM exige optimización.
- **Acción a tomar:**
  - Deserializar directamente desde la porción del slice de bytes `buf` (ej. usando `serde_json::from_slice(&buf[0..actual_len])`).

## Resumen de Tareas Inmediatas a Ejecutar:
1. Reemplazar construcción manual de JSON en `query.rs` por Structs serializables con `serde`.
2. Incluir el parámetro `limit` en el payload dentro de `TicketQuery::all`.
3. Mejorar el manejo de la memoria estática asignada (los buffers 64KB/16KB) y manejar los errores de forma más explícita.
4. Deserializar `serde_json` directamente desde un slice de bytes en lugar de convertir a `String` como paso intermedio en las llamadas al FFI.
