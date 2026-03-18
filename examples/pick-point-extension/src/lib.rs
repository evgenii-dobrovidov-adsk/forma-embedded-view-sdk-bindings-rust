mod extension;

use forma_embedded_view_sdk::spawn_local;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::JsCast;
use web_sys::window;

fn document() -> web_sys::Document {
    window().unwrap().document().unwrap()
}

fn set_text(id: &str, value: &str) {
    if let Some(el) = document().get_element_by_id(id) {
        el.set_text_content(Some(value));
    }
}

fn set_status(msg: &str) {
    set_text("status", msg);
}

fn set_point(point: Option<[f64; 3]>) {
    match point {
        Some(point) => {
            set_text("point-x", &format!("{:.3}", point[0]));
            set_text("point-y", &format!("{:.3}", point[1]));
            set_text("point-z", &format!("{:.3}", point[2]));
        }
        None => {
            set_text("point-x", "-");
            set_text("point-y", "-");
            set_text("point-z", "-");
        }
    }
}

fn set_button_disabled(id: &str, disabled: bool) {
    if let Some(el) = document().get_element_by_id(id) {
        if let Ok(button) = el.dyn_into::<web_sys::HtmlButtonElement>() {
            button.set_disabled(disabled);
        }
    }
}

async fn init_app() -> Result<(), forma_embedded_view_sdk::SdkError> {
    set_point(None);
    set_status("Ready. Click \"Pick point\" to start, then click in the 3D scene.");

    let pick_cb = Closure::wrap(Box::new(move || {
        set_button_disabled("btn-pick", true);
        set_status("Picking point... Click in the scene or press Esc to cancel.");

        spawn_local(async move {
            match extension::pick_point().await {
                Ok(Some(point)) => {
                    set_point(Some(point));
                    set_status(&format!(
                        "Picked point: {:.3}, {:.3}, {:.3}",
                        point[0], point[1], point[2]
                    ));
                }
                Ok(None) => {
                    set_status("Point selection cancelled.");
                }
                Err(e) => {
                    set_status(&format!("Error picking point: {e}"));
                }
            }

            set_button_disabled("btn-pick", false);
        });
    }) as Box<dyn FnMut()>);

    document()
        .get_element_by_id("btn-pick")
        .unwrap()
        .add_event_listener_with_callback("click", pick_cb.as_ref().unchecked_ref())?;
    pick_cb.forget();

    let clear_cb = Closure::wrap(Box::new(move || {
        set_point(None);
        set_status("Cleared last picked point.");
    }) as Box<dyn FnMut()>);

    document()
        .get_element_by_id("btn-clear")
        .unwrap()
        .add_event_listener_with_callback("click", clear_cb.as_ref().unchecked_ref())?;
    clear_cb.forget();

    Ok(())
}

#[no_mangle]
pub extern "C" fn pick_point_extension_main() {
    spawn_local(async {
        if let Err(e) = init_app().await {
            web_sys::console::error_1(&format!("Extension init failed: {e}").into());
            set_status(&format!("Failed to initialize: {e}"));
        }
    });
}
