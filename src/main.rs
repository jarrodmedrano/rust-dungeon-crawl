#![warn(clippy::pedantic)]

mod map;
mod map_builder;
mod camera;
mod spawner;
mod components;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
}

use prelude::*;
use crate::systems::build_scheduler;

struct State {
    ecs : World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        // skip player's first room when adding a monster
        map_builder.rooms
            .iter()
            .skip(1)
            // mapping an iterator passes each entry into a closure (returning a different type of result)
            // transforms one type of iterator into another
            .map(|r| r.center())
            // closure on each location
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));
        // add player and its components to ECS
        spawn_player(&mut ecs, map_builder.player_start);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        Self {
            ecs,
            resources,
            systems: build_scheduler()
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        // makes keyboard available6
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs,&mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT,
                                   "dungeonfont.png")
        .build()?;


    main_loop(context, State::new())
}