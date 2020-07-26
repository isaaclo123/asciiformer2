use crate::debug;
use crate::entities::{Bullet, Direction, Entity, Player};

use crate::genindex::GenIndex;
use crate::map::Map;
use crate::renderer;
use crate::vectors::Vector;
use std::io::{Read, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};
use termion::event::*;

use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use termion::event::Key;
use termion::input::TermRead;
use termion::{clear, cursor, terminal_size};

pub struct Game<'a, R, W> {
    width: u16,
    height: u16,
    stdin: &'a mut R,
    stdout: &'a mut W,
    map: Rc<RefCell<Map>>,
    origin: Vector<u16>,
    // player: RefCell<Player<'a>>,
    player_id: u16,
    gen_index: RefCell<GenIndex<Rc<RefCell<dyn Entity>>>>,
}

impl<'a, R: Read, W: Write> Game<'a, R, W> {
    pub fn new(stdin: &'a mut R, stdout: &'a mut W, map_file: impl AsRef<Path>) -> Self {
        let (width, height) = terminal_size().expect("Failed to get Terminal Size");
        let map = Map::load_from_file(map_file).unwrap();

        let offset_width = (width - map.width) / 2;
        let offset_height = (height - map.height) / 2;

        let gen_index: RefCell<GenIndex<Rc<RefCell<dyn Entity>>>> = RefCell::new(GenIndex::new(50));

        let player = Rc::new(RefCell::new(Player::new(20.1, 20.7, "My Name")));

        let player_id = gen_index.borrow_mut().alloc_index(player).unwrap();

        Game {
            width: width,
            height: height,
            map: Rc::new(RefCell::new(map)),
            stdin: stdin,
            stdout: stdout,
            origin: Vector {
                x: offset_width,
                y: offset_height,
            },
            player_id: player_id as u16,
            gen_index,
            // player: RefCell::new(Player::new(20.1, 20.7, "My Name")),
        }
    }

    pub fn start(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();

        self.map.borrow().draw_map(self.stdout, self.origin);

        let mut before = Instant::now();
        let interval = 60;

        loop {
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

    fn run(
        &self,
        gen_index: &RefMut<GenIndex<Rc<RefCell<dyn Entity>>>>,
        direction_opt: Option<Direction>,
    ) {
        // self.player
        //     .borrow_mut()
        //     .clear(self.stdout, self.origin, Rc::clone(&self.map));
        let mut player = gen_index
            .get(self.player_id as usize)
            .as_ref()
            .unwrap()
            .borrow_mut();

        if let Some(d) = direction_opt {
            debug::write(&format!("Action Being Taken in Run {:?}", d));
            player.action(d)
        }
        player.update();
        player.collide(&self.map); // TODO borrow if possible
                                   // self.player.borrow_mut().update();
                                   // self.player.borrow_mut().collide(Rc::clone(&self.map));

        // self.player.borrow_mut().draw(self.stdout, self.origin);
    }

    fn add_bullet(coord: (u16, u16)) {
        // let entity = self
        //     .gen_index
        //     .borrow()
        //     .get(self.player_id as usize)
        //     .unwrap();
        // let mut player: Player;

        // let bullet = Bullet::new(player.point, Vector::new(coord.0 as f32, coord.1 as f32));
        // self.gen_index
        //     .borrow_mut()
        //     .alloc_index(Rc::new(RefCell::new(bullet)));
    }

    pub fn update(&mut self) -> bool {
        let debug = false;

        let gen_index = self.gen_index.borrow_mut();
        let entities = gen_index.to_vec();

        for e in &entities {
            debug::write(&format!(
                "ENTITY TO CLEAR {} {}",
                e.borrow().to_string(),
                e.borrow().get_point()
            ));
            renderer::clear(e, self.stdout, self.origin, &self.map);
        }

        // Clear

        if let Some(c) = self.stdin.events().next() {
            match c.unwrap() {
                Event::Key(ke) => match ke {
                    Key::Char('q') => {
                        // self.game_over();
                        return false;
                    }
                    Key::Up => self.run(&gen_index, Some(Direction::Up)),
                    Key::Down => self.run(&gen_index, Some(Direction::Down)),
                    Key::Right => self.run(&gen_index, Some(Direction::Right)),
                    Key::Left => self.run(&gen_index, Some(Direction::Left)),
                    _ => {
                        if debug {
                            self.run(&gen_index, None)
                        }
                    }
                },
                Event::Mouse(me) => match me {
                    MouseEvent::Press(_, a, b) => {
                        let x = a as i16 - self.origin.x as i16 - 1;
                        let y = b as i16 - self.origin.y as i16 - 1;

                        let sym = if self.map.borrow().get(x, y).is_some() {
                            'W'
                        } else {
                            ' '
                        };

                        debug::write(&format!("cursor ({}, {}) {}", x, y, sym));
                    }
                    _ => (), // MouseEvent::Release(a, b) | MouseEvent::Hold(a, b) => {
                             //     // write!(self.stdout, "{}x", cursor::Goto(a, b)).unwrap();
                             // }
                },
                _ => self.run(&gen_index, None),
            }
        } else {
            if !debug {
                self.run(&gen_index, None)
            }
        }

        for e in &entities {
            renderer::draw(e, self.stdout, self.origin);
        }

        // if !debug {
        //     self.run(&gen_index, None)
        // }

        //     // run here when not debug
        //     self.player.borrow_mut().draw(self.stdout, self.origin);
        // }

        self.stdout.flush().unwrap();

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
}
