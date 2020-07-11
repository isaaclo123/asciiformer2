use crate::consts::{EntityType, TEXTURE_MAP};
use crate::entities::{Entity, Player};
use crate::map::Map;
use crate::vectors::Vector;
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};
use termion::event::*;

use termion::event::Key;
use termion::input::{Events, TermRead};
use termion::{clear, color, cursor, terminal_size};

pub struct Game<'a, R, W> {
    width: u16,
    height: u16,
    stdin: &'a mut R,
    stdout: &'a mut W,
    map: Map,
}

impl<'a, R: Read, W: Write> Game<'a, R, W> {
    pub fn new(stdin: &'a mut R, stdout: &'a mut W, map_file: impl AsRef<Path>) -> Game<'a, R, W> {
        let (width, height) = terminal_size().expect("Failed to get Terminal Size");
        let map = Map::load_from_file(map_file).unwrap();

        Game {
            width: width,
            height: height,
            map: map,
            stdin: stdin,
            stdout: stdout,
        }
    }

    pub fn start(&mut self) {
        self.display_map();

        let mut before = Instant::now();
        let interval = 100;

        write!(self.stdout, "{}", cursor::Hide).unwrap();

        loop {
            //self.display_map();
            let now = Instant::now();
            let dt = (now.duration_since(before).subsec_nanos() / 1_000_000) as u64;

            if dt < interval {
                sleep(Duration::from_millis(interval - dt));
                continue;
            }

            before = now;

            if !self.update() {
                return;
            }
        }
    }

    pub fn update(&mut self) -> bool {
        let mut p = Player {
            name: "hi",
            point: Vector { x: 3.6, y: 3.6 },
            velocity: Vector { x: 0.0, y: 0.0 },
        };

        let offset_width = (self.width - self.map.width) / 2;
        let offset_height = (self.height - self.map.height) / 2;

        p.draw(
            self.stdout,
            Vector {
                x: offset_width,
                y: offset_height,
            },
        );
        // p.update();

        if let Some(c) = self.stdin.events().next() {
            match c.unwrap() {
                Event::Key(Key::Char('q')) => {
                    self.game_over();
                    return false;
                }
                Event::Mouse(me) => match me {
                    MouseEvent::Press(_, a, b)
                    | MouseEvent::Release(a, b)
                    | MouseEvent::Hold(a, b) => {
                        write!(self.stdout, "{}x", cursor::Goto(a, b)).unwrap();
                    }
                },
                _ => return true,
            }
            self.stdout.flush().unwrap();
        }

        true
    }

    pub fn game_over(&mut self) {
        write!(
            self.stdout,
            "{clear}Thank you for playing!",
            clear = clear::All
        )
        .unwrap();

        write!(self.stdout, "{}", cursor::Show).unwrap();
    }

    pub fn display_map(&mut self) {
        write!(
            self.stdout,
            "{clear}",
            // Full screen clear.
            clear = clear::All,
        )
        .unwrap();

        let offset_width = (self.width - self.map.width) / 2;
        let offset_height = (self.height - self.map.height) / 2;

        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let texture = match TEXTURE_MAP.get(self.map.level[y as usize][x as usize]) {
                    Some(e) => *e,
                    // TODO should throw error if character not found
                    None => ' ',
                };
                write!(
                    self.stdout,
                    "{goto}{texture}",
                    // add 1 as cursor is 1-indexed
                    goto = cursor::Goto(offset_width + x + 1, offset_height + y + 1),
                    texture = texture
                )
                .unwrap();
            }
        }

        self.stdout.flush().unwrap();
    }
}
