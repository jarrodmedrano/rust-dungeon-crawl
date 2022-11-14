use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(
    ecs: &SubWorld,
    // request read only access to the mouse position
    #[resource] mouse_pos: &Point,
    // request read only access to the camera
    #[resource] camera: &Camera
) {
    let mut positions = <(Entity, &Point, &Name)>::query();
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    positions
        .iter(ecs)
        // filter to reduce contents to only include elements whose Point position is equal to
        // the current mouse cursor position stored in Map_pos
        .filter(|(_, pos, _)| **pos == map_pos)
        .for_each(|(entity, _, name) | {
            // mouse position is in coordinates that align with the monster's layer
            // tooltip layer is 4 x bigger so multiply the mouse position by 4 to get screen position for
            // tooltip layer
            let screen_pos = *mouse_pos * 4;
            // use entry_ref() to access an entity’s components from outside a query
            let display = if let Ok(health) = ecs.entry_ref(*entity)
                // Start by requesting the entry and unwrapping it
                .unwrap()
                // Use get_component() to retrieve a component from the ECS without a query
                .get_component::<Health>()
            {
                format!("{} : {} hp", &name.0, health.current)
            } else {
                // if let if there is no match we can run code in the else block
                name.0.clone()
            };
            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}