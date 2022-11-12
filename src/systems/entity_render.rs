use crate::prelude::*;

#[system]
// requests read only access to the point and render component
// point tells where entity is
#[read_component(Point)]
// render describes appearance of the point
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    // starts a new drawbatch in each system to render to terminal
    let mut draw_batch = DrawBatch::new(); // <callout id="co.ecs.entity_render.newbatch" />
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    // query for all entities with a point and render component
    <(&Point, &Render)>::query()// <callout id="co.ecs.entity_render.query" />
        .iter(ecs)// <callout id="co.ecs.entity_render.iter" />
        .for_each(|(pos, render)| {// <callout id="co.ecs.entity_render.foreach" />
            draw_batch.set(// <callout id="co.ecs.entity_render.set" />
                           *pos - offset,
                           render.color,
                           render.glyph
            );
        }
        );
    draw_batch.submit(5000).expect("Batch error");
}