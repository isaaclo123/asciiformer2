use crate::consts::{EntityType, TEXTURE_MAP};
use crate::debug;
use crate::entities::{Direction, Entity, Player, Wall};
use crate::map::Map;
use crate::vectors::Vector;
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use std::rc::Rc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use termion::event::*;

use std::cell::RefCell;
use termion::event::Key;
use termion::input::{Events, TermRead};
use termion::{clear, color, cursor, terminal_size};

pub struct GameInfo<'a, R, W> {
    pub width: u16,
    pub height: u16,
    pub stdin: &'a mut R,
    pub stdout: &'a mut W,
    pub map: Map<'a, R, W>,
    pub origin: Vector<u16>,
}

pub struct Game<'a, R, W> {
    pub game_info: Rc<RefCell<GameInfo<'a, R, W>>>,
    player: RefCell<Player<'a, R, W>>,
    // self_refcell: RefCell<&'a mut Self>,
}

impl<'a, R: Read, W: Write> Game<'a, R, W> {
    pub fn new(stdin: &'a mut R, stdout: &'a mut W, map_file: impl AsRef<Path>) -> Game<'a, R, W> {
        let (width, height) = terminal_size().expect("Failed to get Terminal Size");

        let game_info = RefCell::new(GameInfo {
            width: width,
            height: height,
            map: Map::new(),
            stdin: stdin,
            stdout: stdout,
            // origin: Vector {
            //     x: offset_width,
            //     y: offset_height,
            // },
            origin: Vector::new(0, 0),
        });

        game_info
            .borrow_mut()
            .map
            .load_from_file(game_info, map_file)
            .unwrap();

        let map = game_info.borrow().map;

        game_info.borrow_mut().origin =
            Vector::new((width - map.width) / 2, (height - map.height) / 2);

        let game_rc = Rc::new(game_info);

        Game {
            game_info: game_rc,
            player: RefCell::new(Player::new(game_rc, Vector { x: 20.1, y: 20.7 }, "My Name")),
        }
    }

    pub fn start(&mut self) {
        write!(self.game_info.borrow().stdout, "{}", cursor::Hide).unwrap();

        self.draw_map();
        // self.game_info.borrow().stdout.flush().unwrap();

        let mut before = Instant::now();
        let interval = 60;

        loop {
            // debug::clear(self.game_info.borrow().stdout);
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
            self.player
                .borrow_mut()
                .action(self.game_info.borrow().stdout, d);
        } else {
            debug::write(self.game_info.borrow().stdout, "RUN NONE");
        }
        self.player.borrow_mut().update();
        self.player.borrow_mut().wall_collide();
        self.player.borrow_mut().draw();
        self.game_info.borrow().stdout.flush().unwrap();
    }

    pub fn update(&mut self) -> bool {
        let debug = false;

        self.player.borrow_mut().clear();

        if let Some(c) = self.game_info.borrow_mut().stdin.events().next() {
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
                    Key::Char(' ') => self.run(Some(Direction::Up)),
                    Key::Char('w') => self.run(Some(Direction::Up)),
                    Key::Char('s') => self.run(Some(Direction::Down)),
                    Key::Char('d') => self.run(Some(Direction::Right)),
                    Key::Char('a') => self.run(Some(Direction::Left)),
                    _ => {
                        if debug {
                            self.run(None);
                        }
                    }
                },
                Event::Mouse(me) => match me {
                    MouseEvent::Press(_, a, b) => {
                        let x = a as i16 - self.game_info.borrow().origin.x as i16 - 1;
                        let y = b as i16 - self.game_info.borrow().origin.y as i16 - 1;

                        let sym = if self
                            .game_info
                            .borrow()
                            .map
                            .get((x as u16, y as u16))
                            .is_some()
                        {
                            'W'
                        } else {
                            ' '
                        };

                        debug::write(
                            self.game_info.borrow().stdout,
                            &format!("cursor ({}, {}) {}", x, y, sym),
                        );
                    }
                    _ => (), // MouseEvent::Release(a, b) | MouseEvent::Hold(a, b) => {
                             //     // write!(self.game_info.borrow().stdout, "{}x", cursor::Goto(a, b)).unwrap();
                             // }
                },
                _ => self.run(None),
            }
            self.game_info.borrow().stdout.flush().unwrap();
        }

        if !debug {
            self.player.borrow_mut().clear();
            self.run(None);
        }

        // run here when not debug
        self.player.borrow_mut().draw();

        true
    }

    pub fn game_over(&mut self) {
        write!(
            self.game_info.borrow().stdout,
            "{clear}Thank you for playing!",
            clear = clear::All
        )
        .unwrap();

        write!(self.game_info.borrow().stdout, "{}", cursor::Show).unwrap();
    }

    pub fn draw_map(&mut self) {
        write!(
            self.game_info.borrow().stdout,
            "{clear}",
            // Full screen clear.
            clear = clear::All,
        )
        .unwrap();

        for (_, entity) in &self.game_info.borrow().map.level {
            entity.draw()
        }
    }
}
