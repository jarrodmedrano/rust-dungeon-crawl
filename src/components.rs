pub use crate::prelude::*;

// deeriving clone so we can clone it and debug so we can debug it
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    //color pair is a background and foreground color
    pub color : ColorPair,
    pub glyph : FontCharType
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Monster;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity : Entity,
    pub destination : Point
}

// structs can also be tuples
#[derive(Clone, PartialEq)]
pub struct Name(pub String);