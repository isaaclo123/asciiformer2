use super::debug;
use super::entities::{Entity, EntitySync, Wall};
use super::helpers::unlock;
use super::renderer;

use super::vectors::Vector;
use ndarray::{array, Array2};
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
// pub type MapData = HashMap<(u16, u16), Arc<Mutex<dyn Entity>>>;
pub type MapData = Array2<Option<i16>>;
pub type OuterMapData = HashMap<(i16, i16), i16>;

pub type MapSync = Arc<Mutex<Map>>;

// #[derive(Debug)]
pub struct Map {
    pub level: MapData,
    pub level_outer: OuterMapData,
    pub width: u16,
    pub height: u16,
    // pub players: Vec
}

impl Map {
    pub fn new() -> Self {
        Self {
            level: array![[]],
            level_outer: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn get(&self, x: i16, y: i16) -> Option<i16> {
        if x < 0 || y < 0 || x >= self.width as i16 || y >= self.height as i16 {
            return self.level_outer.get(&(x, y));
        }

        self.level
            .get(y as usize)
            .unwrap()
            .get(x as usize)
            .unwrap()
            .as_ref()
    }

    pub fn set(&mut self, x: i16, y: i16, id: i16) -> Result<(), &str> {
        if x < 0 || y < 0 || x >= self.width as i16 || y >= self.height as i16 {
            self.level_outer.insert((x, y), id);
        }

        self.level[y as usize][x as usize] = Some(id);

        return Ok(());
    }

    pub fn del(&mut self, x: i16, y: i16) -> Result<(), &str> {
        if x < 0 || y < 0 || x >= self.width as i16 || y >= self.height as i16 {
            self.level_outer.remove(&(x, y));
        }

        self.level[y as usize][x as usize] = None;

        return Ok(());
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

        for y in 0..self.height {
            for x in 0..self.width {
                if self.level[y as usize][x as usize].is_some() {
                    let wall = Wall::new(x, y);
                    renderer::draw(&wall, stdout, origin);
                }
            }
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

    pub fn load_from_file(&mut self, filename: impl AsRef<Path>) -> Result<(), Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut y = 0;
        let mut x;

        let mut max_x = 0;
        let wall = Wall::new(0, 0);

        for li in reader.lines() {
            x = 0;

            let line = li?;
            let chars = line.chars();
            self.level.push(vec![]);

            for ch in chars {
                let result = match LEVEL_MAP.get(&ch) {
                    Some(e) => match *e {
                        EntityType::WALL => wall.get_id(),
                        _ => None,
                    },
                    // TODO should throw error if character not found
                    None => None,
                };

                if result.is_none() {
                    debug::write(&format!("NONE at {} {}", x, y));
                }
                self.level[y].push(result);
                x += 1;
            }

            if x > max_x {
                max_x = x
            }

            y += 1;
        }

        let max_y = y;

        self.height = max_y as u16;
        self.width = max_x as u16;

        debug::write(&format!("height {} width {}", self.height, self.width));
        // debug::write(&format!("{:?}", self.level));

        Ok(())
    }
}
