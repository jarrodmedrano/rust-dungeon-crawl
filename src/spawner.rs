use crate::prelude::*;

// mutable reference to world and location
pub fn spawn_player(ecs : &mut World, pos : Point) {
    // components created by calling push
    ecs.push(
        (
            // Tag component Player
            Player,
            // player's position
            pos,
            // player's appearance
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph : to_cp437('@')
            },
            Health { current: 20, max: 20 }
       )
    );
}

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos : Point
) {
    ecs.push(
        (Enemy,
            //Start; spawn_monster
         pos,
         Render {
             color: ColorPair::new(WHITE, BLACK),
             glyph: match rng.range(0, 4) {
                 0 => to_cp437('E'),
                 1 => to_cp437('O'),
                 2 => to_cp437('o'),
                 _ => to_cp437('g'),
             }
         },
         MovingRandomly{}
        )
            //End: spawn_monster
    );
}
