use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
// obtain read only access to the Map resource
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    // creates a new Query with writeable access to Point and read-only access to MovingRandomly
    let mut movers = <(&mut Point, &MovingRandomly)>::query();
    movers
        .iter_mut(ecs)
        .for_each(|(pos, _)| {
        let mut rng = RandomNumberGenerator::new();
            // randomly choose a direction in which to move and store the delta
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0,1),
        } + *pos;

        if map.can_enter_tile(destination) {
            *pos = destination;
        }
    }
        );
}