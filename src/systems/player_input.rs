use crate::prelude::*;

// procedural macro transforms function into player_input_system
#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    // like a world but can only see components we request
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState
)
{
    // player entity and destination are destructured from the iterator results
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    if let Some(key) = *key {

        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        //START: get_player_dest
        let (player_entity, destination) = players
            .iter(ecs)
            // transforms iterator data into other iterator data
            // stops first time we return Some data
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)) )
            // retrieves our tuple from the iterator chain and stores it in player_entity and destination
            .unwrap();
        //END: get_player_dest

        //START: find_enemies
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        let mut did_something = false;
        if delta.x !=0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| {
                    **pos == destination
                })  // if any matched, it will skip this step
                //END: find_enemies
                //START: for_each
                .for_each(|(entity, _) | {
                    //if loop executes we know we are targeting enemy and standing in destination tile
                    hit_something = true;
                    did_something = true;
                    commands
                        // send wants to attack message ot legion command
                        .push(((), WantsToAttack{
                            attacker: player_entity,
                            target : *entity,
                        }));
                });
            //END: for_each

            //START: hit_something
            if !hit_something {
                did_something = true;
                commands
                    .push(((), WantsToMove{
                        entity: player_entity,
                        destination
                    }));
            }
            // grant health back if player rests
            if !did_something {
                if let Ok(mut health) = ecs
                    .entry_mut(player_entity)
                    .unwrap()
                    .get_component_mut::<Health>()
                {
                    // add health back
                    health.current = i32::min(health.max, health.current+1);
                    println!("Health after healing: {}", health.current);
                }
            }
        }

        *turn_state = TurnState::PlayerTurn;
        //END: hit_something
        }
}