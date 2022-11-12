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
            }
       )
    );
}