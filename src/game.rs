use crate::consts::{EntityType, TEXTURE_MAP};
use crate::debug;
use crate::entities::{Direction, Entity, Player};
use crate::map::Map;
use crate::vectors::Vector;
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};
use termion::event::*;

use std::cell::RefCell;
use termion::event::Key;
use termion::input::{Events, TermRead};
use termion::{clear, color, cursor, terminal_size};

pub struct Game<'a, R, W> {
    width: u16,
    height: u16,
    stdin: &'a mut R,
    stdout: &'a mut W,
    map: Map,
    origin: Vector<u16>,
    player: RefCell<Player<'a>>,
}

impl<'a, R: Read, W: Write> Game<'a, R, W> {
    pub fn new(stdin: &'a mut R, stdout: &'a mut W, map_file: impl AsRef<Path>) -> Game<'a, R, W> {
        let (width, height) = terminal_size().expect("Failed to get Terminal Size");
        let map = Map::load_from_file(map_file).unwrap();

        let offset_width = (width - map.width) / 2;
        let offset_height = (height - map.height) / 2;

        Game {
            width: width,
            height: height,
            map: map,
            stdin: stdin,
            stdout: stdout,
            origin: Vector {
                x: offset_width,
                y: offset_height,
            },
            player: RefCell::new(Player::new(20.1, 20.7, "My Name")),
        }
    }

    pub fn start(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();

        self.draw_map();
        // self.stdout.flush().unwrap();

        let mut before = Instant::now();
        let interval = 60;

        loop {
            // debug::clear(self.stdout);
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
            // if !self.update() {
            //     return;
            // }
        }
    }

    fn run(&mut self, direction_opt: Option<Direction>) {
        if let Some(d) = direction_opt {
            self.player.borrow_mut().action(self.stdout, d);
        } else {
            debug::write(self.stdout, "RUN NONE");
        }
        self.player.borrow_mut().update(self.stdout);
        self.player
            .borrow_mut()
            .wall_collide(self.stdout, &self.map.level);
        self.player.borrow_mut().draw(self.stdout, self.origin);
        self.stdout.flush().unwrap();
    }

    pub fn update(&mut self) -> bool {
        self.player
            .borrow_mut()
            .clear(self.stdout, self.origin, &self.map.level);

        if let Some(c) = self.stdin.events().next() {
            match c.unwrap() {
                Event::Key(ke) => match ke {
                    Key::Char('q') => {
                        self.game_over();
                        return false;
                    }
                    Key::Up => self.run(Some(Direction::Up)),
                    Key::Down => self.run(Some(Direction::Down)),
                    Key::Right => self.run(Some(Direction::Right)),
                    Key::Left => self.run(Some(Direction::Left)),
                    _ => self.run(None),
                },
                Event::Mouse(me) => match me {
                    MouseEvent::Press(_, a, b) => {
                        let x = a as i16 - self.origin.x as i16 - 1;
                        let y = b as i16 - self.origin.y as i16 - 1;

                        let sym = if self.map.level.get(&(x as u16, y as u16)).is_some() {
                            'W'
                        } else {
                            ' '
                        };

                        debug::write(self.stdout, &format!("cursor ({}, {}) {}", x, y, sym));
                    }
                    _ => (), // MouseEvent::Release(a, b) | MouseEvent::Hold(a, b) => {
                             //     // write!(self.stdout, "{}x", cursor::Goto(a, b)).unwrap();
                             // }
                },
                _ => self.run(None),
            }
            self.stdout.flush().unwrap();
        }

        // run here when not debug
        self.player.borrow_mut().draw(self.stdout, self.origin);

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

    pub fn draw_map(&mut self) {
        write!(
            self.stdout,
            "{clear}",
            // Full screen clear.
            clear = clear::All,
        )
        .unwrap();

        for (_, entity) in &self.map.level {
            entity.draw(self.stdout, self.origin)
        }
    }
}
