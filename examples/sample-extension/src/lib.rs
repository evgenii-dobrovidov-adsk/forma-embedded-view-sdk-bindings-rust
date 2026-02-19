use forma_embedded_view_sdk::FORMA;
use js_sys::{Array, Float32Array, Object, Reflect, Uint8Array};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

#[derive(Serialize)]
struct CategoryRequest {
    category: String,
}

fn document() -> web_sys::Document {
    window().unwrap().document().unwrap()
}

fn set_status(msg: &str) {
    if let Some(el) = document().get_element_by_id("status") {
        el.set_text_content(Some(msg));
    }
}

fn get_color() -> (u8, u8, u8) {
    let doc = document();
    let input = doc
        .get_element_by_id("color-picker")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();
    let hex = input.value();
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(255);
    (r, g, b)
}

async fn load_building_paths() -> Result<Vec<String>, JsValue> {
    let req = serde_wasm_bindgen::to_value(&CategoryRequest {
        category: "building".into(),
    })?;
    let promise = FORMA.with(|f| f.geometry_api().get_paths_by_category(&req));
    let result = JsFuture::from(promise).await?;
    let arr: Array = result.dyn_into()?;
    let mut paths = Vec::with_capacity(arr.length() as usize);
    for i in 0..arr.length() {
        if let Some(s) = arr.get(i).as_string() {
            paths.push(s);
        }
    }
    Ok(paths)
}

async fn get_selection() -> Result<Vec<String>, JsValue> {
    let promise = FORMA.with(|f| f.selection().get_selection());
    let result = JsFuture::from(promise).await?;
    let arr: Array = result.dyn_into()?;
    let mut paths = Vec::with_capacity(arr.length() as usize);
    for i in 0..arr.length() {
        if let Some(s) = arr.get(i).as_string() {
            paths.push(s);
        }
    }
    Ok(paths)
}

async fn get_triangles(path: &str) -> Result<Float32Array, JsValue> {
    let req = Object::new();
    Reflect::set(&req, &"path".into(), &path.into())?;
    let promise = FORMA.with(|f| f.geometry_api().get_triangles(Some(&req)));
    let result = JsFuture::from(promise).await?;
    Ok(result.dyn_into()?)
}

async fn color_selected_buildings(building_paths: &[String]) -> Result<u32, JsValue> {
    let selected = get_selection().await?;
    let (r, g, b) = get_color();
    let mut colored = 0u32;

    for path in &selected {
        if !building_paths.contains(path) {
            continue;
        }

        let position = get_triangles(path).await?;
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

        let req = Object::new();
        Reflect::set(&req, &"id".into(), &path.into())?;
        Reflect::set(&req, &"geometryData".into(), &geometry_data)?;

        let promise = FORMA.with(|f| f.render().update_mesh(&req));
        JsFuture::from(promise).await?;
        colored += 1;
    }

    Ok(colored)
}

async fn reset_render() -> Result<(), JsValue> {
    let promise = FORMA.with(|f| f.render().cleanup());
    JsFuture::from(promise).await?;
    Ok(())
}

async fn init_app() -> Result<(), JsValue> {
    set_status("Loading buildings...");

    let building_paths = load_building_paths().await?;
    let count = building_paths.len();

    if let Some(el) = document().get_element_by_id("building-count") {
        el.set_text_content(Some(&count.to_string()));
    }
    set_status("Ready. Select buildings in the scene, pick a color, and click \"Color\".");

    let paths_for_color = building_paths.clone();
    let color_cb = Closure::wrap(Box::new(move || {
        let paths = paths_for_color.clone();
        wasm_bindgen_futures::spawn_local(async move {
            set_status("Coloring...");
            match color_selected_buildings(&paths).await {
                Ok(n) => set_status(&format!("Colored {n} building(s).")),
                Err(e) => set_status(&format!("Error: {e:?}")),
            }
        });
    }) as Box<dyn FnMut()>);

    document()
        .get_element_by_id("btn-color")
        .unwrap()
        .add_event_listener_with_callback("click", color_cb.as_ref().unchecked_ref())?;
    color_cb.forget();

    let reset_cb = Closure::wrap(Box::new(move || {
        wasm_bindgen_futures::spawn_local(async move {
            match reset_render().await {
                Ok(()) => set_status("Reset complete."),
                Err(e) => set_status(&format!("Error: {e:?}")),
            }
        });
    }) as Box<dyn FnMut()>);

    document()
        .get_element_by_id("btn-reset")
        .unwrap()
        .add_event_listener_with_callback("click", reset_cb.as_ref().unchecked_ref())?;
    reset_cb.forget();

    Ok(())
}

#[wasm_bindgen(start)]
pub fn main() {
    wasm_bindgen_futures::spawn_local(async {
        if let Err(e) = init_app().await {
            web_sys::console::error_1(&format!("Extension init failed: {e:?}").into());
            set_status(&format!("Failed to initialize: {e:?}"));
        }
    });
}
