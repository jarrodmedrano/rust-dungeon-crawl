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
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        //  Match can only match on inclusive ranges, requiring the equals, (..=), symbol.
        1..=8 => goblin(),
        _ => orc()
    };

    ecs.push(
        (Enemy,
            //Start; spawn_monster
         pos,
         Render {
             color: ColorPair::new(WHITE, BLACK),
             glyph,
         },
         MovingRandomly{},
         Health{current: hp, max: hp},
            // slightly different tuple construct
         Name(name)
        )
            //End: spawn_monster
    );
}

fn goblin() -> (i32, String, FontCharType) {
    // number is the hp count
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}