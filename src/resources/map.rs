use specs::world::Index;
use std::collections::HashMap;
use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

#[derive(Default, Debug)]
pub struct Map {
    walls: Vec<Vec<bool>>,
    entities: Vec<Vec<Option<Index>>>,
    outer_entities: HashMap<(i16, i16), Index>,
    pub height: usize,
    pub width: usize,
}

const WALL_CHAR: char = '#';
pub const WALL: char = '█';

impl Map {
    pub fn wall_get_unchecked(&self, x: i16, y: i16) -> bool {
        self.walls[y as usize][x as usize]
    }

    pub fn wall_get(&self, x: i16, y: i16) -> bool {
        if x < 0 || y < 0 || x >= self.width as i16 || y >= self.height as i16 {
            return false;
        }

        self.walls[y as usize][x as usize]
    }

    pub fn entity_get(&self, x: i16, y: i16) -> Option<&Index> {
        if x < 0 || y < 0 || x >= self.width as i16 || y >= self.height as i16 {
            return self.outer_entities.get(&(y, x));
        }

        self.entities[y as usize][x as usize].as_ref()
    }

    pub fn entity_set(&mut self, x: i16, y: i16, id: Index) -> () {
        if x < 0 || y < 0 || x >= self.width as i16 || y >= self.height as i16 {
            self.outer_entities.insert((y, x), id);
        }

        self.entities[y as usize][x as usize] = Some(id);
    }

    pub fn entity_del(&mut self, x: i16, y: i16) -> () {
        if x < 0 || y < 0 || x >= self.width as i16 || y >= self.height as i16 {
            self.outer_entities.remove(&(y, x));
        }

        self.entities[y as usize][x as usize] = None;
    }

    pub fn new(filename: impl AsRef<Path>) -> Result<Self, Error> {
        let file = File::open(filename)?;
        let buf = BufReader::new(file);

        let lines: Vec<String> = buf
            .lines()
            .map(|l| l.expect("Could not parse line in map"))
            .collect();

        let height = lines.len();
        let width = lines.iter().fold(0, |acc, x| max(acc, x.len()));

        let mut walls = vec![vec![false; width]; height];

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == WALL_CHAR {
                    walls[y][x] = true;
                }
            }
        }

        Ok(Self {
            entities: vec![vec![None; width]; height],
            outer_entities: HashMap::new(),
            walls,
            height,
            width,
        })
    }
}
