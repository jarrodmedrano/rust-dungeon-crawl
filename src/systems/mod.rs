use crate::prelude::*;

mod map_render;
mod entity_render;
mod player_input;
mod collisions;
mod randome_move;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        // Flushing after collision detection ensures that any deleted entities are really gone
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}