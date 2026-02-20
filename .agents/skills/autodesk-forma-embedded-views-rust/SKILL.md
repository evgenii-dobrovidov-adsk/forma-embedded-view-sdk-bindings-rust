---
name: autodesk-forma-embedded-views-rust
description: "Rust/WASM bindings and patterns for building Autodesk Forma embedded-view extensions in Rust compiled to WebAssembly, using idiomatic Rust wrappers over the forma-embedded-view-sdk npm package."
---

# Autodesk Forma Embedded Views — Rust / WASM

## Overview

This is a **companion skill** to `autodesk-forma-embedded-views`, which is the **source of truth** for Forma extension logic, SDK concepts, API semantics, and what is possible with the Forma Embedded View SDK. This skill provides additional reference on how to **translate those JS/TS patterns into the Rust programming language** using the idiomatic Rust bindings in this repository.

Refer to `autodesk-forma-embedded-views` for:
- What each Forma API does and when to use it
- Extension architecture, lifecycle, and capabilities
- Request/response shapes, parameter meanings, and expected behavior

Refer to **this skill** for:
- The Rust equivalents of JS/TS SDK calls
- Project setup, build tooling, and HTML importmap configuration for Rust/WASM extensions
- Concrete Rust types for requests and responses
- Extension structure patterns (separating SDK logic from DOM glue)

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
│   └── forma-embedded-view-sdk/            # Rust bindings crate
│       ├── Cargo.toml                      # lib crate, crate-type = ["rlib"]
│       └── src/
│           ├── lib.rs                      # EmbeddedViewSdk wrapper + forma() accessor
│           ├── error.rs                    # SdkError type
│           ├── types.rs                    # All concrete request/response types
│           ├── subscription.rs             # Subscription / VoidSubscription (RAII unsubscribe)
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
│           ├── sun.rs                      # Forma.sun bindings
│           └── terrain.rs                  # Forma.terrain (+ groundTexture)
└── examples/
    └── sample-extension/                   # Working example: "Color Selected Buildings"
        ├── Cargo.toml                      # cdylib crate
        ├── build.sh                        # wasm-pack build --target web
        ├── src/
        │   ├── lib.rs                      # DOM glue and entry point
        │   └── extension.rs                # SDK logic (no DOM dependencies)
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

#### 1. Architecture: Hiding WASM Internals

The bindings crate exposes a **pure Rust API**. All `wasm_bindgen`, `js_sys`, and `web_sys` types are hidden from consumers. Each module follows this pattern internally:

- A private `js` submodule contains the raw `wasm_bindgen` extern blocks
- A public wrapper struct (e.g., `GeoDataApi`) holds the raw JS type and exposes idiomatic Rust methods
- All methods that call JS Promises are `async fn` returning `Result<T, SdkError>`
- Request/response types are concrete Rust structs with `serde::Serialize`/`Deserialize`
- Typed arrays (`Float32Array`, `Uint8Array`, `ArrayBuffer`) are automatically converted to/from `Vec<f32>`, `Vec<u8>`, etc.
- JS `Date` objects are converted to/from ISO 8601 strings
- DOM types like `HTMLCanvasElement` and `MessagePort` are wrapped in opaque Rust types with useful accessor methods

Extension code **does not need** any additional dependencies beyond this bindings crate.

#### 2. Including the JS SDK from esm.sh

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

This `<script type="importmap">` tag must appear **before** any `<script type="module">` tags in the HTML. Both entries are required — the first maps the base package, and the `/auto` entry maps the auto-initialization subpath that the singleton uses internally. When updating the SDK version, change the version number in **both** URLs simultaneously.

Without this importmap the browser will fail to resolve the bare specifiers and the WASM module will not initialize.

See `examples/sample-extension/web/index.html` for a complete working example.

#### 3. The `forma()` Accessor

The primary entry point is the `forma()` function, which returns an `EmbeddedViewSdk` wrapper around the pre-configured JS singleton (`import { Forma } from "forma-embedded-view-sdk/auto"`):

```rust
use forma_embedded_view_sdk::forma;

let sdk = forma();

// Synchronous methods
let project_id = sdk.get_project_id();
let region = sdk.get_region();

// Async methods — just .await directly
let location = sdk.project().get_geo_location().await?;
let paths = sdk.geometry().get_paths_by_category(&request).await?;
```

#### 4. Manual SDK Construction

For cases where auto-initialization isn't desired:

```rust
use forma_embedded_view_sdk::EmbeddedViewSdk;

let sdk = EmbeddedViewSdk::new(None)?; // or Some(&config)
let region = sdk.get_region();
```

#### 5. Async Pattern

All SDK methods that correspond to JS Promises are `async fn` returning `Result<T, SdkError>`. No manual `JsFuture::from()` or `serde_wasm_bindgen::to_value()` is needed:

```rust
use forma_embedded_view_sdk::types::*;

async fn get_project_location() -> forma_embedded_view_sdk::Result<Option<[f64; 2]>> {
    let sdk = forma_embedded_view_sdk::forma();
    sdk.project().get_geo_location().await
}
```

#### 6. Concrete Request/Response Types

All request and response types are defined in `forma_embedded_view_sdk::types`. Import them with `use forma_embedded_view_sdk::types::*;`:

```rust
use forma_embedded_view_sdk::types::*;

async fn load_buildings() -> forma_embedded_view_sdk::Result<Vec<String>> {
    let sdk = forma_embedded_view_sdk::forma();
    sdk.geometry()
        .get_paths_by_category(&GetPathsByCategoryRequest {
            category: "building".into(),
        })
        .await
}
```

Key types include:
- `GetPathsByCategoryRequest`, `GetTrianglesRequest`, `GetFootprintRequest`
- `MeshRequest`, `GeometryData` (with `Vec<f32>` position, `Option<Vec<u8>>` color)
- `GlbRenderRequest` (with `Vec<u8>` for GLB binary data)
- `GeoDataUploadRequest`, `GeoDataType`, `GeoLocation`, `Licensing`
- `LibraryItem`, `LibraryItemData`, `LibraryStatus`
- `CameraState`, `CameraMoveRequest`, `CameraCaptureRequest`
- `CaptureResult` (opaque wrapper with `.to_data_url()`, `.width()`, `.height()`)
- `MessagePortHandle` (opaque wrapper with `.post_message()`)
- `AuthConfig`, `AccessTokenResponse`
- `SunDateRequest`, `ElevationRequest`, `TerrainBbox`
- And many more — see `types.rs` for the full list.

#### 7. Subscription Callbacks

Subscription methods accept native Rust closures. The returned `Subscription` handle automatically unsubscribes when dropped:

```rust
let sdk = forma_embedded_view_sdk::forma();

let subscription = sdk.selection().subscribe(|paths: Vec<String>| {
    // handle selection change
}).await?;

// Unsubscribes automatically when `subscription` is dropped.
// Or call explicitly:
subscription.unsubscribe();
```

For camera:
```rust
let subscription = sdk.camera().subscribe(|state: CameraState| {
    println!("Camera at {:?}", state.position);
}).await?;
```

#### 8. Binary Data Handling

The crate automatically converts between Rust `Vec<f32>` / `Vec<u8>` and JS typed arrays:

- **Sending** `MeshRequest` — `GeometryData.position` (`Vec<f32>`) becomes `Float32Array`, `GeometryData.color` (`Option<Vec<u8>>`) becomes `Uint8Array`
- **Sending** `GlbRenderRequest` — `glb` (`Vec<u8>`) becomes `ArrayBuffer`
- **Sending** `UploadFileRequest` — `data` (`Vec<u8>`) becomes `ArrayBuffer`
- **Receiving** `geometry().get_triangles()` — `Float32Array` becomes `Vec<f32>`
- **Receiving** `elements().blobs().get()` — `ArrayBuffer` becomes `Vec<u8>`
- **Receiving** `elements().representations().volume_mesh()` — `ArrayBuffer` becomes `Vec<u8>`
- **Receiving** `extensions().storage().get_binary_object()` — returns `Vec<u8>`

#### 9. DOM Manipulation via web-sys

Access the DOM through `web_sys` in your extension's glue code:

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

Create a new crate as `crate-type = ["cdylib"]`. Note: the bindings crate handles all JS interop and re-exports `spawn_local`, so you only need `wasm-bindgen` and `web-sys` for DOM glue:

```toml
[package]
name = "my-extension"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
forma-embedded-view-sdk = "0.1"  # requires a workspace [patch.crates-io] or git dependency
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["Document", "Element", "HtmlElement", "Window", "console"] }
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

#### Step 2: Organize the extension code

Separate SDK logic from DOM glue. Create two files:

**`src/extension.rs`** — pure SDK logic, no DOM imports:

```rust
use forma_embedded_view_sdk::types::*;

pub async fn load_building_paths() -> forma_embedded_view_sdk::Result<Vec<String>> {
    let forma = forma_embedded_view_sdk::forma();
    forma
        .geometry()
        .get_paths_by_category(&GetPathsByCategoryRequest {
            category: "building".into(),
        })
        .await
}

pub async fn reset_render() -> forma_embedded_view_sdk::Result<()> {
    let forma = forma_embedded_view_sdk::forma();
    forma.render().cleanup().await
}
```

**`src/lib.rs`** — DOM glue and entry point:

```rust
mod extension;

use forma_embedded_view_sdk::spawn_local;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    spawn_local(async {
        if let Err(e) = init_app().await {
            web_sys::console::error_1(&format!("Init failed: {e}").into());
        }
    });
}

async fn init_app() -> Result<(), forma_embedded_view_sdk::SdkError> {
    let paths = extension::load_building_paths().await?;
    // ... set up UI, wire event handlers ...
    Ok(())
}
```

#### Step 3: Create the HTML entry point

The HTML file must include an **importmap** that maps the SDK's bare module specifiers to CDN URLs:

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

All methods below are accessed through `forma().<api>().<method>(&request)` and are `async fn` returning `Result<T, SdkError>` unless otherwise noted.

| Rust accessor | Module | Key methods |
|---|---|---|
| `.analysis()` | `analysis` | `list`, `trigger_noise`, `trigger_sun`, `get_sun_analysis`, `get_noise_analysis`, `get_ground_grid` |
| `.area_metrics()` | `area_metrics` | `calculate` |
| `.auth()` | `auth` | `configure` (sync), `acquire_token_silent`, `acquire_token_popup`, `acquire_token_overlay`, `refresh_current_token` |
| `.camera()` | `camera` | `move_to`, `switch_perspective`, `capture` → `CaptureResult`, `get_current` → `CameraState`, `subscribe` |
| `.colorbar()` | `colorbar` | `add`, `remove` |
| `.design_tool()` | `design_tool` | `get_point` → `Option<Vec3>`, `get_polygon` → `Option<Vec<Vec3>>`, `get_extruded_polygon`, `get_line`, `on_edit_start`, `on_edit_end` |
| `.elements()` | `elements` | `get`, `get_by_path`, `get_world_transform` → `Transform`, `edit_properties`; sub-APIs: `.floor_stack()`, `.representations()`, `.blobs()` |
| `.extensions()` | `extensions` | `invoke_endpoint`; sub-API: `.storage()` with `set_object`, `get_text_object`, `get_binary_object` → `Vec<u8>`, `list_objects`, `delete_object` |
| `.generators()` | `generators` | `put`, `list` |
| `.geo_data()` | `geo_data` | `upload` → `LibraryItem` |
| `.geometry()` | `geometry` | `get_paths_by_category` → `Vec<String>`, `get_paths_for_virtual_elements` → `Vec<String>`, `get_footprint`, `get_triangles` → `Vec<f32>`, `get_paths_inside_polygons` → `Vec<String>` |
| `.integrate_elements()` | `integrate` | `create_element_hierarchy`, `create_element_v2` → `UrnResult`, `update_element_v2`, `batch_ingest_elements_v2`, `upload_file` → `FileUploadResult`, `create_urn` (sync) |
| `.library()` | `library` | `create_item` → `LibraryItem`, `update_item` → `LibraryItem`, `delete_item` |
| `.predictive_analysis()` | `predictive_analysis` | `get_wind_parameters`, `predict_wind` |
| `.project()` | `project` | `get` → `Project`, `get_geo_location` → `Option<[f64; 2]>` |
| `.proposal()` | `proposal` | `get_root_urn` → `String`, `get_id` → `String`, `add_element` → `PathResult`, `replace_element`, `remove_element`, `replace_terrain`, `update_elements`, `subscribe`, `await_proposal_persisted`, `get_all`, `get`, `create`, `update`, `delete`, `duplicate`, `switch` |
| `.render()` | `render` | `hide_element`, `hide_elements_batch`, `unhide_element`, `unhide_elements_batch`, `set_elements_visibility`, `unhide_all_elements`, `add_mesh` → `IdResult`, `update_mesh`, `remove`, `cleanup`; sub-APIs: `.glb()`, `.geojson()`, `.element_colors()` |
| `.selection()` | `selection` | `get_selection` → `Vec<String>`, `subscribe` |
| `.sun()` | `sun` | `get_date` → `String` (ISO 8601), `set_date` |
| `.terrain()` | `terrain` | `get_bbox` → `TerrainBbox`, `get_elevation_at` → `f64`, `get_pads`, `add_pads`, `apply_pads`; sub-API: `.ground_texture()` |

Top-level `EmbeddedViewSdk` methods (accessed directly on `forma()`):

| Method | Returns | Description |
|---|---|---|
| `.get_project_id()` | `String` | Current project ID |
| `.get_extension_id()` | `String` | Current extension ID |
| `.get_region()` | `String` | Deployment region |
| `.get_embedded_view_id()` | `String` | Embedded view ID |
| `.origin()` | `String` | Host origin |
| `EmbeddedViewSdk::get_host_origin()` | `String` | Static: get host origin |
| `.ping()` | `Result<()>` | Health check |
| `.get_presentation_unit_system()` | `Result<UnitSystem>` | Unit system |
| `.get_can_edit()` | `Result<bool>` | Edit permission check |
| `.get_can_view_hub()` | `Result<bool>` | Hub view permission |
| `.get_can_edit_hub()` | `Result<bool>` | Hub edit permission |
| `.open_floating_panel(&options)` | `Result<()>` | Open a floating panel |
| `.close_embedded_view(&options)` | `Result<()>` | Close this embedded view |
| `.on_embedded_view_state_change(callback)` | `Result<Subscription>` | Subscribe to state changes |
| `.on_locale_update(callback)` | `Result<Subscription>` | Subscribe to locale changes |
| `.create_message_port(&options)` | `Result<MessagePortHandle>` | Create a MessagePort |
| `.on_message_port(callback)` | `Subscription` | Listen for message ports |

### Key Dependencies

Extensions depend on these crates:

| Crate | Purpose |
|---|---|
| `forma-embedded-view-sdk` | The bindings crate from this repo (handles JS interop internally and re-exports `spawn_local`) |
| `wasm-bindgen` | Rust ↔ JS FFI (needed for `#[wasm_bindgen(start)]` and `Closure`) |
| `web-sys` | DOM/Web API bindings for extension UI (enable features as needed) |

**Not needed as direct dependencies** (used internally by the bindings crate): `js-sys`, `serde`, `serde-wasm-bindgen`, `serde_json`.

### Build Tooling

- **Rust target:** `wasm32-unknown-unknown` (install via `rustup target add wasm32-unknown-unknown`)
- **wasm-pack:** builds the crate and generates JS glue (`wasm-pack build --target web --out-dir web/pkg --no-typescript`)
- **No bundler required:** the HTML importmap + ESM `import init from "./pkg/..."` pattern works without any JS bundler

### Common Patterns Reference

**Get all building paths:**

```rust
use forma_embedded_view_sdk::types::*;

async fn load_building_paths() -> forma_embedded_view_sdk::Result<Vec<String>> {
    let forma = forma_embedded_view_sdk::forma();
    forma
        .geometry()
        .get_paths_by_category(&GetPathsByCategoryRequest {
            category: "building".into(),
        })
        .await
}
```

**Get current selection:**

```rust
async fn get_selection() -> forma_embedded_view_sdk::Result<Vec<String>> {
    let forma = forma_embedded_view_sdk::forma();
    forma.selection().get_selection().await
}
```

**Render a colored mesh overlay:**

```rust
use forma_embedded_view_sdk::types::*;

async fn color_mesh(path: &str, r: u8, g: u8, b: u8) -> forma_embedded_view_sdk::Result<()> {
    let forma = forma_embedded_view_sdk::forma();
    let position = forma
        .geometry()
        .get_triangles(Some(&GetTrianglesRequest {
            path: path.to_string(),
        }))
        .await?;

    let num_vertices = position.len() / 3;
    let mut color_buf = vec![0u8; num_vertices * 4];
    for i in 0..num_vertices {
        color_buf[i * 4] = r;
        color_buf[i * 4 + 1] = g;
        color_buf[i * 4 + 2] = b;
        color_buf[i * 4 + 3] = 255;
    }

    forma
        .render()
        .update_mesh(&MeshRequest {
            id: path.to_string(),
            geometry_data: GeometryData {
                position,
                color: Some(color_buf),
            },
            transform: None,
        })
        .await
}
```

**Clear all render overlays:**

```rust
async fn reset_render() -> forma_embedded_view_sdk::Result<()> {
    forma_embedded_view_sdk::forma().render().cleanup().await
}
```

**Subscribe to selection changes:**

```rust
async fn watch_selection() -> forma_embedded_view_sdk::Result<forma_embedded_view_sdk::Subscription> {
    let forma = forma_embedded_view_sdk::forma();
    forma.selection().subscribe(|paths: Vec<String>| {
        // handle selection change — paths is already Vec<String>
    }).await
}
```

**Move the camera:**

```rust
use forma_embedded_view_sdk::types::*;

async fn fly_to(position: [f64; 3], target: [f64; 3]) -> forma_embedded_view_sdk::Result<()> {
    let forma = forma_embedded_view_sdk::forma();
    forma
        .camera()
        .move_to(&CameraMoveRequest {
            position: Some(position),
            target: Some(target),
            transition: Some(true),
        })
        .await
}
```

**Upload GeoJSON buildings:**

```rust
use forma_embedded_view_sdk::types::*;

async fn upload_buildings(geojson: FeatureCollection) -> forma_embedded_view_sdk::Result<LibraryItem> {
    let forma = forma_embedded_view_sdk::forma();
    forma
        .geo_data()
        .upload(&GeoDataUploadRequest {
            data: geojson,
            data_type: GeoDataType::Buildings,
            geo_location: Some(GeoLocation {
                ref_point: [0.0, 0.0],
                srid: 4326,
            }),
            licensing: None,
        })
        .await
}
```

**DOM event handler bridging async work:**

```rust
use forma_embedded_view_sdk::spawn_local;
use wasm_bindgen::prelude::*;

let handler = Closure::wrap(Box::new(move || {
    spawn_local(async move {
        // async SDK calls here
    });
}) as Box<dyn FnMut()>);

element.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref()).unwrap();
handler.forget();
```

### Translating from JS/TS SDK to Rust

After consulting `autodesk-forma-embedded-views` for the correct SDK logic and API calls, apply these translations to convert them into Rust:

| JavaScript / TypeScript | Rust equivalent |
|---|---|
| `import { Forma } from "forma-embedded-view-sdk/auto"` | `use forma_embedded_view_sdk::forma;` then `let sdk = forma();` |
| `await Forma.project.get()` | `sdk.project().get().await?` |
| `await Forma.geometry.getPathsByCategory({category: "building"})` | `sdk.geometry().get_paths_by_category(&GetPathsByCategoryRequest { category: "building".into() }).await?` |
| `await Forma.geometry.getTriangles({path})` | `sdk.geometry().get_triangles(Some(&GetTrianglesRequest { path })).await?` → `Vec<f32>` |
| `await Forma.render.updateMesh({id, geometryData: {position, color}})` | `sdk.render().update_mesh(&MeshRequest { id, geometry_data: GeometryData { position, color }, transform: None }).await?` |
| `await Forma.render.glb.add({glb: arrayBuffer})` | `sdk.render().glb().add(&GlbRenderRequest { id, glb: vec_u8, transform: None }).await?` |
| `await Forma.selection.getSelection()` | `sdk.selection().get_selection().await?` → `Vec<String>` |
| `Forma.selection.subscribe(callback)` | `sdk.selection().subscribe(\|paths: Vec<String>\| { ... }).await?` → `Subscription` |
| `Forma.camera.move({position, target})` | `sdk.camera().move_to(&CameraMoveRequest { ... }).await?` |
| `Forma.sun.getDate()` | `sdk.sun().get_date().await?` → `String` (ISO 8601) |
| `Forma.sun.setDate({date: new Date(...)})` | `sdk.sun().set_date(&SunDateRequest { date: "2024-06-21T12:00:00Z".into() }).await?` |
| `Forma.camera.capture({})` | `sdk.camera().capture(&request).await?` → `CaptureResult` with `.to_data_url()` |
| `Forma.geoData.upload({data, dataType, ...})` | `sdk.geo_data().upload(&GeoDataUploadRequest { ... }).await?` → `LibraryItem` |
| `Forma.integrateElements.createElementV2(...)` | `sdk.integrate_elements().create_element_v2(&request).await?` → `UrnResult` |
| `{ key: value }` (JS object literal) | Concrete Rust struct from `types.rs` |
| `result.someField` | Directly available as struct field |

### Gotchas and Tips

1. **`spawn_local`** — use `forma_embedded_view_sdk::spawn_local(async { ... })` (or `use forma_embedded_view_sdk::spawn_local;`) to run async code from synchronous contexts (e.g., DOM event handlers, `#[wasm_bindgen(start)]`) without a direct `wasm-bindgen-futures` dependency. This is unavoidable at the boundary between sync DOM callbacks and async SDK calls.
2. **`camera().move_to()`** — named `move_to` because `move` is a Rust keyword.
3. **Subscription lifetime** — `Subscription` auto-unsubscribes on `Drop`. Store it in a long-lived location to keep the subscription active. Call `.unsubscribe()` for explicit cleanup.
4. **Closure lifetime for DOM events** — use `.forget()` for long-lived DOM event callbacks. This intentionally leaks memory to prevent the closure from being dropped while JS still holds a reference.
5. **Feature flags** — `web-sys` requires explicit feature flags for every DOM type you use. Add them to `Cargo.toml` as needed.
6. **Error handling** — async SDK calls return `Result<T, SdkError>`. Use `?` for propagation.
7. **importmap is required** — the wasm-bindgen-generated JS glue does `import ... from "forma-embedded-view-sdk"`, which the browser resolves via the HTML `<script type="importmap">`.
8. **SDK version** — the importmap URLs pin the SDK version (currently `0.91.0`). Update both `forma-embedded-view-sdk` and `forma-embedded-view-sdk/auto` entries together.
9. **No bundler needed** — `wasm-pack build --target web` produces ESM that works directly in the browser with the importmap. Serve with any static file server.
10. **Separate SDK logic from DOM glue** — put all `forma_embedded_view_sdk` calls in a dedicated `extension.rs` file, and keep DOM manipulation in `lib.rs`. This makes the SDK logic testable and the boundary clear.
