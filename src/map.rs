use super::entities::{Entity, EntitySync, Wall};
use super::renderer;

use super::vectors::Vector;
use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};
use termion::{clear, cursor, terminal_size};

use super::consts::{EntityType, LEVEL_MAP};

// pub type MapData = Vec<Vec<EntityTypeVal>>;
pub type MapData = HashMap<(u16, u16), Arc<Mutex<dyn Entity>>>;

pub type MapSync = Arc<Mutex<Map>>;

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
    //
    pub fn get(&self, x: i16, y: i16) -> Option<&EntitySync> {
        if x < 0 || y < 0 || x >= self.width as i16 || y >= self.height as i16 {
            return None;
        }

        let pt = (x as u16, y as u16);

        self.level.get(&pt)
    }

    // pub fn get_level(&self) -> &MapData {
    //     &self.level
    // }

    pub fn draw_map(&self, stdout: &mut impl Write, origin: Vector<u16>) {
        write!(
            stdout,
            "{clear}",
            // Full screen clear.
            clear = clear::All,
        )
        .unwrap();

        for (_, e) in &self.level {
            renderer::draw(&*e.lock().unwrap(), stdout, origin);
        }
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
    //         if self.level.get(&point).is_some() {
    //             return false;
    //         }
    //     }
    //     true
    // }

    pub fn load_from_file(filename: impl AsRef<Path>) -> Result<Map, Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut map: MapData = HashMap::new();

        let mut y = 0;
        let mut x;

        let mut max_x = 0;

        for li in reader.lines() {
            x = 0;

            let line = li?;
            let chars = line.chars();

            for ch in chars {
                match LEVEL_MAP.get(&ch) {
                    Some(e) => match *e {
                        EntityType::WALL => {
                            map.insert((x, y), Arc::new(Mutex::new(Wall::new(x, y))));
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
