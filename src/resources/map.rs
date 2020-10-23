use ndarray::{array, Array2};
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
    walls: Array2<bool>,
    entities: Array2<Option<Index>>,
    outer_entities: HashMap<(i16, i16), Index>,
    pub height: usize,
    pub width: usize,
}

const WALL_CHAR: char = '#';

impl Map {
    pub fn wall_get(&self, x: i16, y: i16) -> bool {
        let wall_result = self.walls.get((x as usize, y as usize));

        if let Some(wall) = wall_result {
            return *wall;
        }

        false
    }

    pub fn entity_get(&self, x: i16, y: i16) -> Option<&Index> {
        let entity_result = self.entities.get((x as usize, y as usize));

        if let Some(entity) = entity_result {
            return entity.as_ref();
        }

        self.outer_entities.get(&(x, y))
    }

    pub fn entity_set(&mut self, x: i16, y: i16, entity: Index) -> Result<(), &str> {
        if x < 0 || y < 0 || x >= self.width as i16 || y >= self.height as i16 {
            self.outer_entities.insert((x, y), entity);
        } else {
            *self.entities.get_mut((y as usize, x as usize)).unwrap() = Some(entity);
        }

        return Ok(());
    }

    pub fn entity_del(&mut self, x: i16, y: i16) -> () {
        if x < 0 || y < 0 || x >= self.width as i16 || y >= self.height as i16 {
            self.outer_entities.remove(&(x, y));
        } else {
            *self.entities.get_mut((y as usize, x as usize)).unwrap() = None;
        }
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

        let mut walls = Array2::from_elem((height, width), false);

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == WALL_CHAR {
                    *walls.get_mut((y, x)).unwrap() = true;
                }
            }
        }

        Ok(Self {
            entities: Array2::from_elem((height, width), None),
            outer_entities: HashMap::new(),
            walls,
            height,
            width,
        })
    }
}
