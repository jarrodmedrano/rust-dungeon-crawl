use crate::prelude::*;

// procedural macro transforms function into player_input_system
#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    // like a world but can only see components we request
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    // like a mutable borrow of the camera struct
    #[resource] camera: &mut Camera
)
{
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            // access components with a query
            // queries list one ore more components and return references, mutable if we use &mut
            let mut players = <&mut Point>::query()
                //filters so only Player components
                .filter(component::<Player>());
            // adding filter before the iterator limits types included in query
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            })
        }
    }
}