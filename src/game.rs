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
    origin_float: Vector<f32>,
    // player: RefCell<Player<'a>>,
    player_id: u16,
    gen_index: GenIndex<Rc<RefCell<dyn Entity>>>,
}

impl<'a, R: Read, W: Write> Game<'a, R, W> {
    pub fn new(stdin: &'a mut R, stdout: &'a mut W, map_file: impl AsRef<Path>) -> Self {
        let (width, height) = terminal_size().expect("Failed to get Terminal Size");
        let map = Map::load_from_file(map_file).unwrap();

        let offset_width = (width - map.width) / 2;
        let offset_height = (height - map.height) / 2;

        let mut gen_index: GenIndex<Rc<RefCell<dyn Entity>>> = GenIndex::new(100);

        let player = Rc::new(RefCell::new(Player::new(20.1, 20.7, "My Name")));

        let player_id = gen_index.alloc_index(player).unwrap();

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
            origin_float: Vector {
                x: offset_width as f32,
                y: offset_height as f32,
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

    fn run(&mut self, direction: Direction) {
        // self.player
        //     .borrow_mut()
        //     .clear(self.stdout, self.origin, Rc::clone(&self.map));
        let player_opt = self.gen_index.get(self.player_id as usize).unwrap();
        let mut player = player_opt.borrow_mut();

        match direction {
            Direction::To(end) => {
                self.gen_index
                    .alloc_index(Rc::new(RefCell::new(Bullet::new(player.get_point(), end))));
            }
            Direction::None => (),
            _ => player.action(direction),
        }
        // player.update();
        // player.collide(&self.map); // TODO borrow if possible
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

        for e in &mut self.gen_index {
            renderer::clear(&e, self.stdout, self.origin, &self.map);
        }

        // Clear

        if let Some(c) = self.stdin.events().next() {
            match c.unwrap() {
                Event::Key(ke) => match ke {
                    Key::Char('q') => {
                        // self.game_over();
                        return false;
                    }
                    Key::Up => self.run(Direction::Up),
                    Key::Down => self.run(Direction::Down),
                    Key::Right => self.run(Direction::Right),
                    Key::Left => self.run(Direction::Left),
                    _ => {
                        if debug {
                            self.run(Direction::None)
                        }
                    }
                },
                Event::Mouse(me) => match me {
                    MouseEvent::Press(_, a, b) => {
                        let x = a as i16 - self.origin.x as i16 - 1;
                        let y = b as i16 - self.origin.y as i16 - 1;

                        // let sym = if self.map.borrow().get(x, y).is_some() {
                        //     'W'
                        // } else {
                        //     ' '
                        // };

                        self.run(Direction::To(Vector::new(x as f32, y as f32)));
                    }
                    _ => (), // MouseEvent::Release(a, b) | MouseEvent::Hold(a, b) => {
                             //     // write!(self.stdout, "{}x", cursor::Goto(a, b)).unwrap();
                             // }
                },
                _ => self.run(Direction::None),
            }
        } else {
            if !debug {
                self.run(Direction::None)
            }
        }

        for e in &mut self.gen_index {
            e.borrow_mut().update();
            e.borrow_mut().collide(&self.map);
            renderer::draw(&e, self.stdout, self.origin);
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
