use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    if map.can_enter_tile(want_move.destination) {
        // uses command to add component batches updates and performed at once
        commands.add_component(want_move.entity, want_move.destination);
        // adding component that already exists replaces the old one
        if ecs.entry_ref(want_move.entity)
            .unwrap()
            // returns a Result
            // check if component exists by calling is_ok
            .get_component::<Player>().is_ok()
        {
            // update player's camera information
            camera.on_player_move(want_move.destination)
        }
    }
}