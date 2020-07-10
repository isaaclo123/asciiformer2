use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
    vec::Vec,
};

use crate::consts::{EntityType, EntityTypeVal, LEVEL_MAP};

pub type MapData = Vec<Vec<EntityTypeVal>>;

#[derive(Debug)]
pub struct Map {
    pub level: MapData,
    pub width: u16,
    pub height: u16,
    // pub players: Vec
}

impl Map {
    pub fn new() -> Map {
        Map {
            level: vec![vec![]],
            width: 0,
            height: 0,
        }
    }

    pub fn load_from_file(filename: impl AsRef<Path>) -> Result<Map, Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut map: MapData = Vec::new();

        for li in reader.lines() {
            let line = li?;
            let chars = line.chars();

            let mut map_line: Vec<EntityTypeVal> = Vec::new();

            for ch in chars {
                let entity = match LEVEL_MAP.get(&ch) {
                    Some(e) => match *e {
                        EntityType::PLAYER => {
                            let _x = 5;
                            EntityType::AIR
                        }
                        _ => e,
                    },
                    // TODO should throw error if character not found
                    None => EntityType::AIR,
                };
                map_line.push(entity);
            }

            map.push(map_line);
        }

        Ok(Map {
            height: map.len() as u16,
            width: map[0].len() as u16,
            level: map,
        })
    }
}
