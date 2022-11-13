use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &SubWorld, commands: &mut CommandBuffer) {
    // creates a new Query with writeable access to Point and read-only access to MovingRandomly
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    movers.iter(ecs).for_each(| (entity, pos, _) | {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;
        let mut attacked = false;
        positions.iter(ecs)
            // queries all entities and lists only ones in the destination tile
            .filter(|(_, target_pos, _)| **target_pos == destination)
            .for_each(|(target, _, _)| {
                // if destination contains an entity, it checks to see if the target has the player component
                if ecs.entry_ref(*target)
                    .unwrap().get_component::<Player>().is_ok()
                {
                    // if it does it sends the want to attack command
                    commands.push(((), WantsToAttack{
                        attacker: *entity,
                        target: *target
                    }));
                }
                // sets to true even if no attack was launched, prevents monster from moving to other monster
                attacked = true;
            });
        // if monster attack it won't also move onto their tile
        if !attacked {
            commands
                .push(((), WantsToMove{ entity: *entity, destination }));
        }
    });

}