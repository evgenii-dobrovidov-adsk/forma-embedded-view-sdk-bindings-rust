use forma_embedded_view_sdk::types::*;
use wasm_bindgen::prelude::*;
use web_sys::window;

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

async fn load_building_paths() -> Result<Vec<String>, forma_embedded_view_sdk::SdkError> {
    let forma = forma_embedded_view_sdk::forma();
    forma
        .geometry()
        .get_paths_by_category(&GetPathsByCategoryRequest {
            category: "building".into(),
        })
        .await
}

async fn get_selection() -> Result<Vec<String>, forma_embedded_view_sdk::SdkError> {
    let forma = forma_embedded_view_sdk::forma();
    forma.selection().get_selection().await
}

async fn get_triangles(path: &str) -> Result<Vec<f32>, forma_embedded_view_sdk::SdkError> {
    let forma = forma_embedded_view_sdk::forma();
    forma
        .geometry()
        .get_triangles(Some(&GetTrianglesRequest {
            path: path.to_string(),
        }))
        .await
}

async fn color_selected_buildings(
    building_paths: &[String],
) -> Result<u32, forma_embedded_view_sdk::SdkError> {
    let forma = forma_embedded_view_sdk::forma();
    let selected = get_selection().await?;
    let (r, g, b) = get_color();
    let mut colored = 0u32;

    for path in &selected {
        if !building_paths.contains(path) {
            continue;
        }

        let position = get_triangles(path).await?;
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
                id: path.clone(),
                geometry_data: GeometryData {
                    position,
                    color: Some(color_buf),
                },
                transform: None,
            })
            .await?;
        colored += 1;
    }

    Ok(colored)
}

async fn reset_render() -> Result<(), forma_embedded_view_sdk::SdkError> {
    let forma = forma_embedded_view_sdk::forma();
    forma.render().cleanup().await
}

async fn init_app() -> Result<(), forma_embedded_view_sdk::SdkError> {
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
                Err(e) => set_status(&format!("Error: {e}")),
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
                Err(e) => set_status(&format!("Error: {e}")),
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
            web_sys::console::error_1(&format!("Extension init failed: {e}").into());
            set_status(&format!("Failed to initialize: {e}"));
        }
    });
}
