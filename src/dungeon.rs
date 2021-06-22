//
// Dungeon generation
//

use crate::{Map, Rect};
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy)]
struct Room {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Room {
    fn distance_to(&self, room: &Room) -> i32 {
        Rect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
        .distance_to(&Rect {
            x: room.x,
            y: room.y,
            width: room.width,
            height: room.height,
        })
    }

    fn stamp(&self, map: &mut Map) {
        map.draw_rect_filled(
            &Rect {
                x: self.x,
                y: self.y,
                width: self.width,
                height: self.height,
            },
            crate::Element::Floor,
        );
        map.draw_rect(
            &Rect {
                x: self.x - 1,
                y: self.y - 1,
                width: self.width + 2,
                height: self.height + 2,
            },
            crate::Element::Wall,
        );
    }
}

pub struct DungeonGenParams {
    room_extra_size: u32,
    num_rooms: u32,
}

impl DungeonGenParams {
    pub fn new() -> Self {
        DungeonGenParams {
            room_extra_size: 4,
            num_rooms: 10,
        }
    }
}

pub fn gen_dungeon(map: &mut Map, params: &DungeonGenParams) {
    let mut r = thread_rng();
    let odd_width = map.width - if map.width % 2 == 0 { 1 } else { 0 };
    let odd_height = map.height - if map.height % 2 == 0 { 1 } else { 0 };

    //
    // Step 1 - generate a bunch of non-overlapping rooms
    //

    let mut rooms = Vec::new();
    (0..params.num_rooms).for_each(|_| {
        // Generate a room
        let size = r.gen_range(1..3 + params.room_extra_size) * 2 + 1;
        let rectangularity = r.gen_range(0..1 + size / 2) * 2;
        let mut width = size;
        let mut height = size;
        if r.gen_range(0..=1) == 0 {
            width += rectangularity;
        } else {
            height += rectangularity;
        }

        let x = r.gen_range(0..(odd_width - width) / 2) * 2 + 1;
        let y = r.gen_range(0..(odd_height - height) / 2) * 2 + 1;

        let room = Room {
            x,
            y,
            width,
            height,
        };

        let count = rooms
            .iter()
            .map(|r| room.distance_to(&r))
            .filter(|x| *x <= 0)
            .count();
        let overlapped = count != 0;
        if !overlapped {
            println!("Adding room: {:?}", room);
            rooms.push(room);
        } else {
            println!("Rejecting room");
        };
    });

    rooms.iter().for_each(|room| {
        map.new_region();
        println!("Stamping room {}", map.current_region);
        room.stamp(map);
    });
}
