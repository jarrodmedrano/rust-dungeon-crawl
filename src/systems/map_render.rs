use crate::prelude::*;

#[system]
// doesn't use any components but needs access to the map and camera
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    // system starts a drawing batch
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y ..= camera.bottom_y {
        for x in camera.left_x .. camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                // call the batch rather than the context
                draw_batch.set(
                    pt - offset,
                    ColorPair::new(
                        WHITE,
                        BLACK
                    ),
                    glyph
                )
            }
        }
    }
    // submit adds to global command list integer is the sort order
    // 0 so the map renders first
    draw_batch.submit(0).expect("Batch error");
}