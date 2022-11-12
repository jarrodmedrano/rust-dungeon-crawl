pub use crate::prelude::*;

// deeriving clone so we can clone it and debug so we can debug it
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
