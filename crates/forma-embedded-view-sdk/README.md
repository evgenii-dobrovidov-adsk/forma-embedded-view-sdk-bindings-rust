# forma-embedded-view-sdk

Rust/WASM bindings for the [forma-embedded-view-sdk](https://www.npmjs.com/package/forma-embedded-view-sdk) npm package.

These bindings allow Rust applications compiled to WebAssembly to interact with the Forma Embedded View SDK — a JavaScript library for creating custom extensions in Autodesk Forma.

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
forma-embedded-view-sdk = { path = "../forma-embedded-view-sdk-bindings-rust" }
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
js-sys = "0.3"
```

### Auto-initialized instance

Use the pre-initialized `Forma` singleton (mirrors `import { Forma } from "forma-embedded-view-sdk/auto"`):

```rust
use forma_embedded_view_sdk::FORMA;
use wasm_bindgen_futures::JsFuture;

async fn example() {
    let project_id = FORMA.with(|forma| forma.get_project_id());

    let project = FORMA.with(|forma| forma.project().get());
    let project = JsFuture::from(project).await.unwrap();
}
```

### Manual initialization

```rust
use forma_embedded_view_sdk::EmbeddedViewSdk;
use wasm_bindgen_futures::JsFuture;

async fn example() {
    let sdk = EmbeddedViewSdk::new(None);
    let region = sdk.get_region();

    let can_edit = JsFuture::from(sdk.get_can_edit()).await.unwrap();
}
```

## API coverage

All public APIs from the SDK are bound:

| Rust module | JS API |
|---|---|
| `analysis` | `Forma.analysis` |
| `area_metrics` | `Forma.areaMetrics` |
| `auth` | `Forma.auth` |
| `camera` | `Forma.camera` |
| `colorbar` | `Forma.colorbar` |
| `design_tool` | `Forma.designTool` |
| `elements` | `Forma.elements` (+ `floorStack`, `representations`, `blobs`) |
| `extensions` | `Forma.extensions` (+ `storage`) |
| `generators` | `Forma.generators` |
| `geo_data` | `Forma.geoData` |
| `geometry` | `Forma.geometry` |
| `integrate` | `Forma.integrateElements` |
| `library` | `Forma.library` |
| `predictive_analysis` | `Forma.predictiveAnalysis` |
| `project` | `Forma.project` |
| `proposal` | `Forma.proposal` |
| `render` | `Forma.render` (+ `glb`, `geojson`, `elementColors`) |
| `selection` | `Forma.selection` |
| `sun` | `Forma.sun` |
| `terrain` | `Forma.terrain` (+ `groundTexture`) |

## Request/response types

Most methods accept and return `JsValue` which can be constructed via `serde_wasm_bindgen` or `js_sys`. Async methods return `js_sys::Promise` which you can await using `wasm_bindgen_futures::JsFuture`.
