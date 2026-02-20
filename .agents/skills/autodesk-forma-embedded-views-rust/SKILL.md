---
name: autodesk-forma-embedded-views-rust
description: "Rust/WASM bindings and patterns for building Autodesk Forma embedded-view extensions in Rust compiled to WebAssembly, using wasm-bindgen bindings for the forma-embedded-view-sdk npm package."
---

# Autodesk Forma Embedded Views — Rust / WASM

## Overview

This is a **companion skill** to `autodesk-forma-embedded-views`, which is the **source of truth** for Forma extension logic, SDK concepts, API semantics, and what is possible with the Forma Embedded View SDK. This skill provides additional reference on how to **translate those JS/TS patterns into the Rust programming language** using the `wasm-bindgen` bindings in this repository.

Refer to `autodesk-forma-embedded-views` for:
- What each Forma API does and when to use it
- Extension architecture, lifecycle, and capabilities
- Request/response shapes, parameter meanings, and expected behavior

Refer to **this skill** for:
- The Rust equivalents of JS/TS SDK calls
- Rust-specific patterns (FORMA singleton access, JsFuture, Closure lifetime, serde serialization)
- Project setup, build tooling, and HTML importmap configuration for Rust/WASM extensions
- Naming differences where Rust keywords required renaming (`geometry_api`, `move_`)

> **DISCLAIMER:** These bindings are unofficial and community-driven. The official SDK is a JS/TS package; these Rust/WASM bindings may break as the upstream SDK evolves.

## When to Use

Use this skill **alongside** `autodesk-forma-embedded-views` when the user wants to:

- Build a new Autodesk Forma extension written in Rust compiled to WASM
- Add features or fix bugs in an existing Rust/WASM Forma extension that uses this bindings crate
- Translate JS/TS Forma SDK calls into their Rust equivalents
- Set up the build pipeline for a Rust/WASM Forma extension (wasm-pack, importmap, HTML entry point)
- Create or modify the `forma-embedded-view-sdk` Rust binding crate itself

First consult `autodesk-forma-embedded-views` to determine the correct SDK APIs and logic for the task, then use this skill to translate that into Rust.

## Repository Structure

```
├── Cargo.toml                              # Workspace root (resolver = "2")
├── crates/
│   └── forma-embedded-view-sdk/            # Rust bindings crate (wasm-bindgen)
│       ├── Cargo.toml                      # lib crate, crate-type = ["rlib"]
│       └── src/
│           ├── lib.rs                      # EmbeddedViewSdk type + FORMA singleton
│           ├── analysis.rs                 # Forma.analysis bindings
│           ├── area_metrics.rs             # Forma.areaMetrics bindings
│           ├── auth.rs                     # Forma.auth bindings
│           ├── camera.rs                   # Forma.camera bindings
│           ├── colorbar.rs                 # Forma.colorbar bindings
│           ├── design_tool.rs              # Forma.designTool bindings
│           ├── elements.rs                 # Forma.elements (+ floorStack, representations, blobs)
│           ├── extensions.rs               # Forma.extensions (+ storage)
│           ├── generators.rs               # Forma.generators bindings
│           ├── geo_data.rs                 # Forma.geoData bindings
│           ├── geometry.rs                 # Forma.geometry bindings
│           ├── integrate.rs                # Forma.integrateElements bindings
│           ├── library.rs                  # Forma.library bindings
│           ├── predictive_analysis.rs      # Forma.predictiveAnalysis bindings
│           ├── project.rs                  # Forma.project bindings
│           ├── proposal.rs                 # Forma.proposal bindings
│           ├── render.rs                   # Forma.render (+ glb, geojson, elementColors)
│           ├── selection.rs                # Forma.selection bindings
│           ├── subscription.rs             # SubscriptionResult helper type
│           ├── sun.rs                      # Forma.sun bindings
│           └── terrain.rs                  # Forma.terrain (+ groundTexture)
└── examples/
    └── sample-extension/                   # Working example: "Color Selected Buildings"
        ├── Cargo.toml                      # cdylib crate
        ├── build.sh                        # wasm-pack build --target web
        ├── src/lib.rs                      # Extension logic
        └── web/
            ├── index.html                  # HTML entry point with importmap
            └── pkg/                        # wasm-pack output (gitignored)
```

## Instructions

### Installing the Bindings Crate

The `forma-embedded-view-sdk` crate is **not published on crates.io**. The host environment is expected to provide a `[patch.crates-io]` entry in the workspace `Cargo.toml` that resolves it from the Git repository automatically, so a plain `cargo add forma-embedded-view-sdk` should work.

If that patch is not present, add the dependency directly from the Git repository:

```bash
cargo add forma-embedded-view-sdk --git https://github.com/evgenii-dobrovidov-adsk/forma-embedded-view-sdk-bindings-rust/tree/main
```

This produces a dependency entry like:

```toml
forma-embedded-view-sdk = { git = "https://github.com/evgenii-dobrovidov-adsk/forma-embedded-view-sdk-bindings-rust", branch = "main" }
```

### Core Concepts

#### 1. Including the JS SDK from esm.sh

The Rust/WASM bindings do **not** bundle the JavaScript SDK themselves. The `wasm-bindgen`-generated JS glue code contains bare `import` statements like `import ... from "forma-embedded-view-sdk"` and `import ... from "forma-embedded-view-sdk/auto"`. The browser needs to know where to find these modules, so **every extension's HTML entry point must include an importmap** that maps these specifiers to the esm.sh CDN:

```html
<script type="importmap">
  {
    "imports": {
      "forma-embedded-view-sdk": "https://esm.sh/forma-embedded-view-sdk@0.91.0",
      "forma-embedded-view-sdk/auto": "https://esm.sh/forma-embedded-view-sdk@0.91.0/auto"
    }
  }
</script>
```

This `<script type="importmap">` tag must appear **before** any `<script type="module">` tags in the HTML. Both entries are required — the first maps the base package, and the `/auto` entry maps the auto-initialization subpath that the `FORMA` singleton uses internally. When updating the SDK version, change the version number in **both** URLs simultaneously.

Without this importmap the browser will fail to resolve the bare specifiers and the WASM module will not initialize.

See `examples/sample-extension/web/index.html` for a complete working example.

#### 2. The FORMA Singleton

The primary entry point is the `FORMA` thread-local static, which mirrors `import { Forma } from "forma-embedded-view-sdk/auto"` in JS. Access it via `FORMA.with(|f| ...)`:

```rust
use forma_embedded_view_sdk::FORMA;

// Synchronous methods — call directly inside .with()
let project_id = FORMA.with(|f| f.get_project_id());
let region = FORMA.with(|f| f.get_region());
let extension_id = FORMA.with(|f| f.get_extension_id());

// Async methods — get the Promise inside .with(), then await outside
let promise = FORMA.with(|f| f.project().get());
let result = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
```

> **Critical pattern:** `FORMA.with()` borrows the SDK for the duration of the closure. You must extract the `js_sys::Promise` (or other return value) *inside* the closure, then `.await` it *outside*. Never `.await` inside `.with()`.

#### 3. Manual SDK Construction

For cases where auto-initialization isn't desired:

```rust
use forma_embedded_view_sdk::EmbeddedViewSdk;

let sdk = EmbeddedViewSdk::new(None); // or Some(&config)
let region = sdk.get_region();
```

#### 4. Async Pattern (Promise → JsFuture)

All SDK methods that return `js_sys::Promise` must be awaited via `JsFuture::from()`:

```rust
use wasm_bindgen_futures::JsFuture;

async fn get_project_location() -> Result<JsValue, JsValue> {
    let promise = FORMA.with(|f| f.project().get_geo_location());
    JsFuture::from(promise).await
}
```

#### 5. Constructing Request Objects

Most SDK methods accept `&JsValue` request parameters. Two approaches:

**A) Using `serde` + `serde_wasm_bindgen` (preferred for structured data):**

```rust
use serde::Serialize;

#[derive(Serialize)]
struct CategoryRequest {
    category: String,
}

let req = serde_wasm_bindgen::to_value(&CategoryRequest {
    category: "building".into(),
}).unwrap();
let promise = FORMA.with(|f| f.geometry_api().get_paths_by_category(&req));
```

**B) Using `js_sys::Object` + `js_sys::Reflect` (for dynamic/ad-hoc objects):**

```rust
use js_sys::{Object, Reflect};

let req = Object::new();
Reflect::set(&req, &"path".into(), &path.into()).unwrap();
let promise = FORMA.with(|f| f.geometry_api().get_triangles(Some(&req)));
```

#### 6. Parsing Response Values

Responses come back as `JsValue`. Cast or deserialize as needed:

```rust
use js_sys::Array;

let result = JsFuture::from(promise).await?;
let arr: Array = result.dyn_into()?;
for i in 0..arr.length() {
    if let Some(s) = arr.get(i).as_string() {
        // use s
    }
}
```

For typed arrays: `result.dyn_into::<js_sys::Float32Array>()`.

#### 7. Event Callbacks and Closures

Subscription/event methods accept `&Closure<dyn FnMut(...)>`. The closure must be leaked with `.forget()` (or stored in a long-lived location) to prevent it from being dropped:

```rust
use wasm_bindgen::prelude::*;

let cb = Closure::wrap(Box::new(move |selection: JsValue| {
    web_sys::console::log_1(&selection);
}) as Box<dyn FnMut(JsValue)>);

let promise = FORMA.with(|f| f.selection().subscribe(&cb));
JsFuture::from(promise).await.unwrap();
cb.forget(); // prevents deallocation
```

For fire-and-forget event listeners on DOM elements:

```rust
let handler = Closure::wrap(Box::new(move || {
    wasm_bindgen_futures::spawn_local(async move {
        // async work here
    });
}) as Box<dyn FnMut()>);

element.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref()).unwrap();
handler.forget();
```

#### 8. DOM Manipulation via web-sys

Access the DOM through `web_sys`:

```rust
use web_sys::window;

fn document() -> web_sys::Document {
    window().unwrap().document().unwrap()
}

fn set_status(msg: &str) {
    if let Some(el) = document().get_element_by_id("status") {
        el.set_text_content(Some(msg));
    }
}
```

Enable specific web-sys features in `Cargo.toml`:

```toml
web-sys = { version = "0.3", features = [
    "Document", "Element", "HtmlElement", "HtmlButtonElement",
    "HtmlInputElement", "Node", "Window", "Text", "Event", "console",
] }
```

### Creating a New Extension

#### Step 1: Set up the Cargo project

Create a new crate as `crate-type = ["cdylib"]`:

```toml
[package]
name = "my-extension"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
forma-embedded-view-sdk = "0.1"  # not on crates.io — requires a workspace [patch.crates-io] entry or a direct git dependency (see "Installing the Bindings Crate")
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["Document", "Element", "HtmlElement", "Window", "console"] }
serde = { version = "1", features = ["derive"] }
serde-wasm-bindgen = "0.6"
```

Add the new crate to the workspace `Cargo.toml`:

```toml
[workspace]
resolver = "2"
members = [
    "crates/forma-embedded-view-sdk",
    "examples/my-extension",
]
```

#### Step 2: Write the extension entry point

Use `#[wasm_bindgen(start)]` to define the WASM entry point, and `spawn_local` for async initialization:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_bindgen_futures::spawn_local(async {
        if let Err(e) = init_app().await {
            web_sys::console::error_1(&format!("Init failed: {e:?}").into());
        }
    });
}

async fn init_app() -> Result<(), JsValue> {
    // Use FORMA.with(|f| ...) to interact with the SDK
    Ok(())
}
```

#### Step 3: Create the HTML entry point

The HTML file must include an **importmap** that maps the SDK's bare module specifiers to CDN URLs. This is how `wasm-bindgen`'s generated JS glue resolves `forma-embedded-view-sdk` imports at runtime:

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <script type="importmap">
    {
      "imports": {
        "forma-embedded-view-sdk": "https://esm.sh/forma-embedded-view-sdk@0.91.0",
        "forma-embedded-view-sdk/auto": "https://esm.sh/forma-embedded-view-sdk@0.91.0/auto"
      }
    }
  </script>
</head>
<body>
  <!-- Your extension UI -->
  <script type="module">
    import init from "./pkg/my_extension.js";
    init();
  </script>
</body>
</html>
```

#### Step 4: Create a build script

```bash
#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
wasm-pack build --target web --out-dir web/pkg --no-typescript
```

#### Step 5: Build and serve

```bash
cd examples/my-extension
./build.sh
cd web
python3 -m http.server 8080
```

Then configure the Forma extension's embedded view URL to `http://localhost:8080`.

### Available API Modules

All methods below are accessed through `FORMA.with(|f| f.<api>().<method>(&request))` and return `js_sys::Promise` unless otherwise noted.

| Rust accessor | Module | Key methods |
|---|---|---|
| `f.analysis()` | `analysis` | `list`, `trigger_noise`, `trigger_sun`, `get_sun_analysis`, `get_noise_analysis`, `get_ground_grid` |
| `f.area_metrics()` | `area_metrics` | `calculate` |
| `f.auth()` | `auth` | `configure` (sync), `acquire_token_silent`, `acquire_token_popup`, `acquire_token_overlay`, `refresh_current_token` |
| `f.camera()` | `camera` | `move_`, `switch_perspective`, `capture`, `get_current`, `subscribe` |
| `f.colorbar()` | `colorbar` | `add`, `remove` |
| `f.design_tool()` | `design_tool` | `get_point`, `get_polygon`, `get_extruded_polygon`, `get_line`, `on_edit_start`, `on_edit_end` |
| `f.elements()` | `elements` | `get`, `get_by_path`, `get_world_transform`, `edit_properties`; sub-APIs: `.floor_stack()`, `.representations()`, `.blobs()` |
| `f.extensions()` | `extensions` | `invoke_endpoint`; sub-API: `.storage()` with `set_object`, `get_text_object`, `get_binary_object`, `list_objects`, `delete_object` |
| `f.generators()` | `generators` | `put`, `list` |
| `f.geo_data()` | `geo_data` | `upload` |
| `f.geometry_api()` | `geometry` | `get_paths_by_category`, `get_paths_for_virtual_elements`, `get_footprint`, `get_triangles`, `get_paths_inside_polygons` |
| `f.integrate_elements()` | `integrate` | `create_element_hierarchy`, `create_element_v2`, `update_element_v2`, `batch_ingest_elements_v2`, `upload_file`, `create_urn` (sync) |
| `f.library()` | `library` | `create_item`, `update_item`, `delete_item` |
| `f.predictive_analysis()` | `predictive_analysis` | `get_wind_parameters`, `predict_wind` |
| `f.project()` | `project` | `get`, `get_geo_location` |
| `f.proposal()` | `proposal` | `get_root_urn`, `get_id`, `add_element`, `replace_element`, `remove_element`, `replace_terrain`, `update_elements`, `subscribe`, `await_proposal_persisted`, `get_all`, `get`, `create`, `update`, `delete`, `duplicate`, `switch` |
| `f.render()` | `render` | `hide_element`, `hide_elements_batch`, `unhide_element`, `unhide_elements_batch`, `set_elements_visibility`, `unhide_all_elements`, `add_mesh`, `update_mesh`, `remove`, `cleanup`; sub-APIs: `.glb()`, `.geojson()`, `.element_colors()` |
| `f.selection()` | `selection` | `get_selection`, `subscribe` |
| `f.sun()` | `sun` | `get_date`, `set_date` |
| `f.terrain()` | `terrain` | `get_bbox`, `get_elevation_at`, `get_pads`, `add_pads`, `apply_pads`; sub-API: `.ground_texture()` |

Top-level `EmbeddedViewSdk` methods (accessed directly on `f`):

| Method | Returns | Description |
|---|---|---|
| `f.get_project_id()` | `String` | Current project ID |
| `f.get_extension_id()` | `String` | Current extension ID |
| `f.get_region()` | `String` | Deployment region |
| `f.get_embedded_view_id()` | `String` | Embedded view ID |
| `f.origin()` | `String` | Host origin |
| `f.ping()` | `Promise` | Health check |
| `f.get_presentation_unit_system()` | `Promise` | Unit system |
| `f.get_can_edit()` | `Promise` | Edit permission check |
| `f.get_can_view_hub()` | `Promise` | Hub view permission |
| `f.get_can_edit_hub()` | `Promise` | Hub edit permission |
| `f.open_floating_panel(&options)` | `Promise` | Open a floating panel |
| `f.close_embedded_view(&options)` | `Promise` | Close this embedded view |
| `f.on_embedded_view_state_change(&closure)` | `Promise` | Subscribe to state changes |
| `f.on_locale_update(&closure)` | `Promise` | Subscribe to locale changes |
| `f.on_embedded_view_closing(&closure)` | `SubscriptionResult` | Subscribe to closing event |
| `f.create_message_port(&options)` | `Promise` | Create a MessagePort |
| `f.on_message_port(&closure)` | `js_sys::Function` | Listen for message ports |
| `EmbeddedViewSdk::get_host_origin()` | `String` | Static: get host origin |

### Key Dependencies

Extensions depend on these crates:

| Crate | Purpose |
|---|---|
| `forma-embedded-view-sdk` | The bindings crate from this repo |
| `wasm-bindgen` | Rust ↔ JS FFI |
| `wasm-bindgen-futures` | `JsFuture`, `spawn_local` for async |
| `js-sys` | JS built-in types (`Promise`, `Array`, `Object`, `Reflect`, `Float32Array`, `Uint8Array`, etc.) |
| `web-sys` | DOM/Web API bindings (enable features as needed) |
| `serde` + `serde-wasm-bindgen` | Serialize Rust structs → `JsValue` for request objects |

### Build Tooling

- **Rust target:** `wasm32-unknown-unknown` (install via `rustup target add wasm32-unknown-unknown`)
- **wasm-pack:** builds the crate and generates JS glue (`wasm-pack build --target web --out-dir web/pkg --no-typescript`)
- **No bundler required:** the HTML importmap + ESM `import init from "./pkg/..."` pattern works without any JS bundler

### Common Patterns Reference

**Get all building paths:**

```rust
use serde::Serialize;

#[derive(Serialize)]
struct CategoryRequest { category: String }

async fn load_building_paths() -> Result<Vec<String>, JsValue> {
    let req = serde_wasm_bindgen::to_value(&CategoryRequest {
        category: "building".into(),
    })?;
    let promise = FORMA.with(|f| f.geometry_api().get_paths_by_category(&req));
    let result = JsFuture::from(promise).await?;
    let arr: js_sys::Array = result.dyn_into()?;
    let mut paths = Vec::with_capacity(arr.length() as usize);
    for i in 0..arr.length() {
        if let Some(s) = arr.get(i).as_string() {
            paths.push(s);
        }
    }
    Ok(paths)
}
```

**Get current selection:**

```rust
async fn get_selection() -> Result<Vec<String>, JsValue> {
    let promise = FORMA.with(|f| f.selection().get_selection());
    let result = JsFuture::from(promise).await?;
    let arr: js_sys::Array = result.dyn_into()?;
    let mut paths = Vec::with_capacity(arr.length() as usize);
    for i in 0..arr.length() {
        if let Some(s) = arr.get(i).as_string() {
            paths.push(s);
        }
    }
    Ok(paths)
}
```

**Render a colored mesh overlay:**

```rust
use js_sys::{Object, Reflect, Float32Array, Uint8Array};

async fn color_mesh(path: &str, r: u8, g: u8, b: u8) -> Result<(), JsValue> {
    let req = Object::new();
    Reflect::set(&req, &"path".into(), &path.into())?;
    let promise = FORMA.with(|f| f.geometry_api().get_triangles(Some(&req)));
    let position: Float32Array = JsFuture::from(promise).await?.dyn_into()?;

    let num_vertices = position.length() / 3;
    let color = Uint8Array::new_with_length(num_vertices * 4);
    let mut buf = vec![0u8; (num_vertices * 4) as usize];
    for i in 0..num_vertices as usize {
        buf[i * 4] = r;
        buf[i * 4 + 1] = g;
        buf[i * 4 + 2] = b;
        buf[i * 4 + 3] = 255;
    }
    color.copy_from(&buf);

    let geometry_data = Object::new();
    Reflect::set(&geometry_data, &"position".into(), &position)?;
    Reflect::set(&geometry_data, &"color".into(), &color)?;

    let mesh_req = Object::new();
    Reflect::set(&mesh_req, &"id".into(), &path.into())?;
    Reflect::set(&mesh_req, &"geometryData".into(), &geometry_data)?;

    let promise = FORMA.with(|f| f.render().update_mesh(&mesh_req));
    JsFuture::from(promise).await?;
    Ok(())
}
```

**Clear all render overlays:**

```rust
async fn reset_render() -> Result<(), JsValue> {
    let promise = FORMA.with(|f| f.render().cleanup());
    JsFuture::from(promise).await?;
    Ok(())
}
```

**Subscribe to selection changes:**

```rust
async fn watch_selection() -> Result<(), JsValue> {
    let cb = Closure::wrap(Box::new(move |val: JsValue| {
        web_sys::console::log_1(&val);
    }) as Box<dyn FnMut(JsValue)>);
    let promise = FORMA.with(|f| f.selection().subscribe(&cb));
    JsFuture::from(promise).await?;
    cb.forget();
    Ok(())
}
```

**Move the camera:**

```rust
async fn fly_to(x: f64, y: f64, z: f64) -> Result<(), JsValue> {
    let req = Object::new();
    let position = Object::new();
    Reflect::set(&position, &"x".into(), &x.into())?;
    Reflect::set(&position, &"y".into(), &y.into())?;
    Reflect::set(&position, &"z".into(), &z.into())?;
    Reflect::set(&req, &"position".into(), &position)?;
    let promise = FORMA.with(|f| f.camera().move_(&req));
    JsFuture::from(promise).await?;
    Ok(())
}
```

### Translating from JS/TS SDK to Rust

After consulting `autodesk-forma-embedded-views` for the correct SDK logic and API calls, apply these translations to convert them into Rust:

| JavaScript / TypeScript | Rust equivalent |
|---|---|
| `import { Forma } from "forma-embedded-view-sdk/auto"` | `use forma_embedded_view_sdk::FORMA;` |
| `Forma.someMethod()` | `FORMA.with(\|f\| f.some_method())` |
| `await Forma.project.get()` | `JsFuture::from(FORMA.with(\|f\| f.project().get())).await` |
| `Forma.geometry.getPathsByCategory({category: "building"})` | `f.geometry_api().get_paths_by_category(&req)` (note: `geometry_api()` not `geometry()`) |
| `Forma.integrateElements.createElementV2(...)` | `f.integrate_elements().create_element_v2(&req)` |
| `Forma.areaMetrics.calculate(...)` | `f.area_metrics().calculate(&req)` |
| `Forma.predictiveAnalysis.predictWind(...)` | `f.predictive_analysis().predict_wind(&req)` |
| `Forma.designTool.getPoint()` | `f.design_tool().get_point()` |
| `Forma.render.glb.add(...)` | `f.render().glb().add(&req)` |
| `Forma.render.geojson.add(...)` | `f.render().geojson().add(&req)` |
| `Forma.render.elementColors.set(...)` | `f.render().element_colors().set(&req)` |
| `Forma.elements.floorStack.createFromFloors(...)` | `f.elements().floor_stack().create_from_floors(&req)` |
| `Forma.extensions.storage.setObject(...)` | `f.extensions().storage().set_object(&req)` |
| `Forma.terrain.groundTexture.add(...)` | `f.terrain().ground_texture().add(&req)` |
| `{ key: value }` (JS object literal) | `serde_wasm_bindgen::to_value(&MyStruct { key: value })` or `Object::new()` + `Reflect::set()` |
| `result.someField` | `Reflect::get(&result, &"someField".into())` |
| `callback => subscription` | `Closure::wrap(Box::new(move \|...\| { ... }))` + `.forget()` |
| `.then(...)` / `await` | `JsFuture::from(promise).await` |

### Gotchas and Tips

1. **Never await inside `FORMA.with()`** — the borrow must end before the `.await` point. Extract the `Promise` first, then await.
2. **`geometry_api()`** not `geometry()` — the Rust binding renames the accessor to avoid conflict with the Rust keyword.
3. **`camera().move_()`** — trailing underscore because `move` is a Rust keyword.
4. **Closure lifetime** — use `.forget()` for long-lived callbacks. This intentionally leaks memory to prevent the closure from being dropped while JS still holds a reference.
5. **`spawn_local`** — use `wasm_bindgen_futures::spawn_local(async { ... })` to run async code from synchronous contexts (e.g., event handlers, `#[wasm_bindgen(start)]`).
6. **Feature flags** — `web-sys` requires explicit feature flags for every DOM type you use. Add them to `Cargo.toml` as needed.
7. **Error handling** — most async calls return `Result<JsValue, JsValue>`. Use `?` for propagation, and `.dyn_into::<T>()` for type narrowing.
8. **importmap is required** — the wasm-bindgen-generated JS glue does `import ... from "forma-embedded-view-sdk"`, which the browser resolves via the HTML `<script type="importmap">`.
9. **SDK version** — the importmap URLs pin the SDK version (currently `0.91.0`). Update both `forma-embedded-view-sdk` and `forma-embedded-view-sdk/auto` entries together.
10. **No bundler needed** — `wasm-pack build --target web` produces ESM that works directly in the browser with the importmap. Serve with any static file server.
