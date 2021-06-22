//
// Dungeon Generation crate
// Copyright (C)2021 Matt Davies
//

mod dungeon;

use std::cmp::min;

pub use dungeon::*;

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
    Wall,
}

//
// Map Cells
//

#[derive(Debug, Clone, Copy)]
pub struct MapCell {
    pub elem: Element,
    pub region: u32,
}

impl MapCell {
    pub fn new(elem: Element, region: u32) -> Self {
        MapCell { elem, region }
    }
}

//
// Map struct
// Represents a rectangle of a map element
//

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub map: Vec<MapCell>,
    current_region: u32,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        Map {
            width,
            height,
            map: vec![MapCell::new(Element::Empty, 0); (width * height) as usize],
            current_region: 0,
        }
    }

    /// Clear the map to empty
    pub fn clear(&mut self) {
        self.map
            .iter_mut()
            .for_each(|x| *x = MapCell::new(Element::Empty, 0));
    }

    /// Given a rect, clip it to the dimensions of the map
    pub fn clip(&self, rect: &Rect) -> Rect {
        Rect {
            x: rect.x,
            y: rect.y,
            width: min(rect.width, self.width - rect.x),
            height: min(rect.height, self.height - rect.y),
        }
    }

    /// Calculate the index into a 1D array from 2D coords
    pub fn coords_to_index(&self, x: u32, y: u32) -> Option<usize> {
        if x < self.width && y < self.height {
            Some((y * self.width + x) as usize)
        } else {
            None
        }
    }

    pub fn new_region(&mut self) {
        self.current_region += 1;
    }

    pub fn draw_rect(&mut self, r: &Rect, elem: Element) {
        if r.width < 3 || r.height < 3 {
            self.draw_rect_filled(r, elem);
        } else {
            // Draw top
            self.draw_rect_filled(
                &Rect {
                    x: r.x,
                    y: r.y,
                    width: r.width,
                    height: 1,
                },
                elem,
            );
            // Draw bottom
            self.draw_rect_filled(
                &Rect {
                    x: r.x,
                    y: r.y + r.height - 1,
                    width: r.width,
                    height: 1,
                },
                elem,
            );
            // Draw left
            self.draw_rect_filled(
                &Rect {
                    x: r.x,
                    y: r.y + 1,
                    width: 1,
                    height: r.height - 2,
                },
                elem,
            );
            // Draw right
            self.draw_rect_filled(
                &Rect {
                    x: r.x + r.width - 1,
                    y: r.y + 1,
                    width: 1,
                    height: r.height - 2,
                },
                elem,
            );
        }
    }

    pub fn draw_rect_filled(&mut self, r: &Rect, elem: Element) {
        let r = self.clip(&r);
        if let Some(mut i) = self.coords_to_index(r.x, r.y) {
            let width = r.width as usize;
            (0..r.height).for_each(|_| {
                let region = self.current_region;
                self.map[i..i + width]
                    .iter_mut()
                    .for_each(|x| *x = MapCell::new(elem, region));
                i += self.width as usize;
            })
        }
    }
}

//
// Generation structures
//

pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn distance_to(&self, other_rect: &Rect) -> i32 {
        let left = self.x as i32;
        let top = self.y as i32;
        let right = left + self.width as i32;
        let bottom = top + self.height as i32;

        let other_left = other_rect.x as i32;
        let other_top = other_rect.y as i32;
        let other_right = other_left + other_rect.width as i32;
        let other_bottom = other_top + other_rect.height as i32;

        let vertical = if top >= other_bottom {
            top - other_bottom
        } else if bottom <= other_top {
            other_top - bottom
        } else {
            -1
        };

        let horizontal = if left >= other_right {
            left - other_right
        } else if right <= other_left {
            other_left - right
        } else {
            -1
        };

        match (horizontal, vertical) {
            (-1, -1) => -1,
            (-1, _) => vertical,
            (_, -1) => horizontal,
            _ => horizontal + vertical,
        }
    }
}
