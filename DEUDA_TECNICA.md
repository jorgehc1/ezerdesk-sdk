# Reporte de Deuda Técnica: `ezerdesk-sdk`

## Estado: Actualizado (Junio 2026)

---

## Issues Resueltos

### ✅ 1. Manejo de Memoria y Límites de Buffers Estáticos — RESUELTO
- **Solución:** Se implementó retry-with-growing-buffer en `http_request()`, `query_data()` y `kv_get_val()`. Los buffers crecen dinámicamente (hasta 5 reintentos) en vez de usar arreglos estáticos.

### ✅ 2. Construcción de JSON por Concatenación de Strings — RESUELTO
- **Solución:** Se reemplazó por estructuras Rust serializables (`TicketQueryPayload`, `SimpleQueryPayload`, `DataWrapper<T>`) con `serde_json::to_string`. El parámetro `limit` ahora se serializa correctamente en `TicketQueryPayload`.

### ✅ 3. Manejo de Errores Silenciosos — RESUELTO
- **Solución:** `host_kv_set` ahora retorna un `u32` (0=éxito, 1=error de escritura). `kv_set_val_checked()` interpreta el código de retorno. `kv_set_val()` loguea errores silenciosamente. El backend (`wasm_engine.gleam`) retorna código de error y loguea el motivo del fallo.

### ✅ 6. Eficiencia en Deserialización — RESUELTO
- **Solución:** Se usa `serde_json::from_slice(&buf[..written])` en vez de convertir a String intermedio.

---

## Issues Activos

### 🔧 4. Dependencia de Ruta Local (Severidad: Baja)
- **Problema:** `Cargo.toml` referencia el macro crate por ruta local: `path = "../ezerdesk-sdk-macros"`.
- **Impacto:** No se puede publicar en crates.io tal como está.
- **Acción:** Definir un workspace explícito o mantener documentado si es para uso interno.

### 🔧 5. Documentación de API — PARCIAL
- **Progreso:** La mayoría de funciones públicas tienen doc comments `///`.
- **Pendiente:** Faltan ejemplos en widget factories (`input()`, `textarea()`, etc.).

---

## Nuevo: Issues Identificados (Junio 2026)

### 🔧 7. `host_publish_response` sobrecargado (Severidad: Alta) — RESUELTO
- **Problema:** `log()` y `to_host_response()` usaban la misma función host. Si `log()` se llamaba después de `to_host_response()`, la respuesta se sobrescribía.
- **Solución:** Se agregó `host_log` como función host separada. `log()` usa `host_log`, `to_host_response()` usa `host_publish_response`.

### 🔧 8. Test bug en `wasm_plugin_test.gleam` (Severidad: Alta) — RESUELTO
- **Problema:** El test hardcodeaba `"print"` (no existe) y omitía `"host_query"` (existe).
- **Solución:** Se usa `wasm_instance.allowed_host_functions` en vez de hardcodear la lista.

### 🔧 9. Backend ignora `limit` del query builder (Severidad: Alta) — RESUELTO
- **Problema:** `plugin_query.gleam` hardcodeaba `LIMIT 50`, haciendo `TicketQuery::limit()` un no-op.
- **Solución:** `execute()` ahora parsea `limit` del JSON y lo pasa al SQL (con clamp 1-200).

### 🔧 10. Version mismatch README vs Cargo.toml (Severidad: Media) — RESUELTO
- **Problema:** README decía `0.1.4`, Cargo.toml decía `0.1.3`.
- **Solución:** README actualizado a `0.1.3`.

---

## Resumen

| # | Issue | Severidad | Estado |
|---|-------|-----------|--------|
| 1 | Buffers estáticos | Alta | ✅ Resuelto |
| 2 | JSON por concatenación | Media | ✅ Resuelto |
| 3 | Errores silenciosos | Media | ✅ Parcial |
| 4 | Ruta local Cargo.toml | Baja | 🔧 Pendiente |
| 5 | Documentación API | Baja | ✅ Parcial |
| 6 | Deserialización ineficiente | Baja | ✅ Resuelto |
| 7 | host_publish_response overloading | Alta | ✅ Resuelto |
| 8 | Test bug | Alta | ✅ Resuelto |
| 9 | Backend ignora limit | Alta | ✅ Resuelto |
| 10 | Version mismatch | Media | ✅ Resuelto |
