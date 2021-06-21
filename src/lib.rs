//
// Dungeon Generation crate
// Copyright (C)2021 Matt Davies
//

use bitmask_enum::bitmask;

//
// Elements
//

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[bitmask(u8)]
pub enum Connection {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy)]
pub enum Element {
    Empty,
    Floor,
    Door(Direction),
}

//
// Map struct
// Represents a rectangle of a map element
//

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub map: Vec<Element>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        Map {
            width,
            height,
            map: vec![Element::Empty; (width * height) as usize],
        }
    }
}
