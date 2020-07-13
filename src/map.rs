use crate::entities::Wall;
use crate::vectors::Vector;
use std::cell::RefCell;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
    vec::Vec,
};

use crate::consts::{EntityType, EntityTypeVal, LEVEL_MAP};

// pub type MapData = Vec<Vec<EntityTypeVal>>;
pub type MapData = HashMap<(u16, u16), Wall>;

// #[derive(Debug)]
pub struct Map {
    pub level: MapData,
    pub width: u16,
    pub height: u16,
    // pub players: Vec
}

impl Map {
    // pub fn new() -> Map {
    //     Map {
    //         level: vec![vec![]],
    //         width: 0,
    //         height: 0,
    //     }
    // }
    //

    pub fn load_from_file(filename: impl AsRef<Path>) -> Result<Map, Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut map: MapData = HashMap::new();

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
                            map.insert((x, y), Wall::new(x, y));
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

        Ok(Map {
            height: max_y,
            width: max_x,
            level: map,
        })
    }
}
