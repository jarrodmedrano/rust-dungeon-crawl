//START: queries
use crate::prelude::*;

// This system requires access to Point, Player, and Enemy.

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {// <callout id="co.ecs.collision_detect.commands" />
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query()
        .filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_pos = *pos);
    // query all enemy positions
    let mut enemies = <(Entity, &Point)>::query()
        .filter(component::<Enemy>());
    enemies.iter(ecs)
        // filter out ones that don't match player position
        // ** removes the references so we can compare with actual value
        // _ ignores the entity we don't need for the filter
        .filter(|(_,pos)| **pos == player_pos)
        // take entity in the tuple, ignore position
        .for_each(|(entity, _) | {
            commands.remove(*entity);
        })
}