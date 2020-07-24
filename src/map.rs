use crate::entities::Wall;
use crate::vectors::Vector;
use std::cell::RefCell;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read, Write},
    path::Path,
};

use crate::consts::{EntityType, EntityTypeVal, LEVEL_MAP};
use crate::game::GameInfo;

// pub type MapData = Vec<Vec<EntityTypeVal>>;
pub type MapData<'a, R, W> = HashMap<(u16, u16), Wall<'a, R, W>>;

// #[derive(Debug)]
pub struct Map<'a, R, W> {
    pub level: MapData<'a, R, W>,
    pub width: u16,
    pub height: u16,
    // pub players: Vec
}

impl<'a, R: Read, W: Write> Map<'a, R, W> {
    pub fn new() -> Self {
        Map {
            level: HashMap::new(),
            width: 0,
            height: 0,
        }
    }
    //
    // pub fn new() -> Map {
    //     Map {
    //         level: vec![vec![]],
    //         width: 0,
    //         height: 0,
    //     }
    // }
    //
    pub fn get(&self, pt: (u16, u16)) -> Option<&Wall<'a, R, W>> {
        self.level.get(&pt)
    }

    // pub fn map_check_wall(&self, location: Vector<f32>) -> bool {
    //     let floor = location.floor_int();
    //     let Vector { x, y } = floor;
    //     let ceil = location.ceil_int();

    //     // Maybe use if statements instead for speed?
    //     let check = match (floor.x != ceil.x, floor.y != ceil.y) {
    //         (true, true) => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
    //         (true, false) => vec![(x, y), (x + 1, y)],
    //         (false, true) => vec![(x, y), (x, y + 1)],
    //         (false, false) => vec![(x, y)],
    //     };

    //     for point in check {
    //         if self.level.borrow().get(&point).is_some() {
    //             return false;
    //         }
    //     }
    //     true
    // }

    pub fn load_from_file(
        &mut self,
        game_info: RefCell<GameInfo<'a, R, W>>,
        filename: impl AsRef<Path>,
    ) -> Result<bool, Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut y = 0;
        let mut x = 0;

        let mut max_x = 0;

        for li in reader.lines() {
            x = 0;

            let line = li?;
            let chars = line.chars();

            for ch in chars {
                match LEVEL_MAP.get(&ch) {
                    Some(e) => match *e {
                        EntityType::WALL => {
                            self.level.insert(
                                (x, y),
                                Wall::new(game_info, Vector::new(x as f32, y as f32)),
                            );
                        }
                        _ => (),
                    },
                    // TODO should throw error if character not found
                    None => (),
                };
                x += 1;
            }

            if x > max_x {
                max_x = x
            }

            y += 1;
        }

        let max_y = y;

        self.height = max_y;
        self.width = max_x;

        Ok(true)
    }
}
