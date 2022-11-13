use crate::prelude::*;

// procedural macro transforms function into player_input_system
#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
    // like a world but can only see components we request
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState
)
{
    let mut players = <(Entity, &Point)>::query()
        .filter(component::<Player>());
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

            // adding filter before the iterator limits types included in query
            players.iter(ecs).for_each(| (entity, pos) | {
                let destination = *pos + delta;
                // adding new entity in commands buffer is the same as pushing
                commands
                    .push(((), WantsToMove{ entity: *entity, destination }));
            });
            *turn_state = TurnState::PlayerTurn;
        }
}