use forma_embedded_view_sdk::types::Vec3;

pub async fn pick_point() -> forma_embedded_view_sdk::Result<Option<Vec3>> {
    let forma = forma_embedded_view_sdk::forma();
    forma.design_tool().get_point().await
}
