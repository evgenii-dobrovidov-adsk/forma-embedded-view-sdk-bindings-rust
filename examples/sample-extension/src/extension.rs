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

pub async fn get_selection() -> forma_embedded_view_sdk::Result<Vec<String>> {
    let forma = forma_embedded_view_sdk::forma();
    forma.selection().get_selection().await
}

pub async fn get_triangles(path: &str) -> forma_embedded_view_sdk::Result<Vec<f32>> {
    let forma = forma_embedded_view_sdk::forma();
    forma
        .geometry()
        .get_triangles(Some(&GetTrianglesRequest {
            path: path.to_string(),
        }))
        .await
}

pub async fn color_selected_buildings(
    building_paths: &[String],
    (r, g, b): (u8, u8, u8),
) -> forma_embedded_view_sdk::Result<u32> {
    let forma = forma_embedded_view_sdk::forma();
    let selected = get_selection().await?;
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

pub async fn reset_render() -> forma_embedded_view_sdk::Result<()> {
    let forma = forma_embedded_view_sdk::forma();
    forma.render().cleanup().await
}
