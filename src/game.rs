use super::helpers::{unlock, wrap};
use crate::debug;
use crate::entities::{Bullet, Direction, Entity, EntitySync, Player};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use termion::async_stdin;

use crate::genindex::GenIndexSync;
use crate::map::{Map, MapSync};
use crate::renderer;
use crate::vectors::Vector;
use std::io::{stdin, Read, Write};
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};
use termion::event::*;

use std::cell::{Cell, RefCell, RefMut};
use std::rc::Rc;
use termion::event::Key;
use termion::input::TermRead;
use termion::{clear, cursor, terminal_size};

pub struct Game<'a, R, W> {
    width: u16,
    height: u16,
    stdin: &'a mut R,
    stdout: &'a mut W,
    map: MapSync,
    origin: Vector<u16>,
    origin_float: Vector<f32>,
    // player: RefCell<Player<'a>>,
    player_id: u16,
    gen_index: GenIndexSync<EntitySync>,
}

impl<'a, R: Read + Send, W: Write + Send> Game<'a, R, W> {
    pub fn new(stdin: &'a mut R, stdout: &'a mut W, map_file: impl AsRef<Path>) -> Self {
        let (width, height) = terminal_size().expect("Failed to get Terminal Size");
        let map = Map::load_from_file(map_file).unwrap();

        let offset_width = (width - map.width) / 2;
        let offset_height = (height - map.height) / 2;

        let mut gen_index: GenIndexSync<EntitySync> = GenIndexSync::new(100);

        let player = wrap(Player::new(20.1, 20.7, "My Name"));
        let player_id = gen_index.alloc_entity(player).unwrap();

        Game {
            width: width,
            height: height,
            map: wrap(map),
            stdin,
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
            gen_index, // player: RefCell::new(Player::new(20.1, 20.7, "My Name")),
        }
    }

    pub fn start(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();

        unlock(&self.map).draw_map(self.stdout, self.origin);

        let mut before = Instant::now();
        let interval = 60;

        // let (sender, receiver) = channel();

        // let stdin = self.stdin;
        let origin = self.origin;
        let stdin = Arc::new(stdin());
        let mut gen_index = self.gen_index.clone();
        let player_id = self.player_id;

        thread::spawn(move || {
            debug::write("Spawned Input Thread");
            // let sender = sender.clone();
            // let my_stdin = Arc::clone(&stdin);
            // let mut stdin = stdin.lock();
            // for c in stdin.unwrap().events() {
            for c in stdin.lock().events() {
                debug::write("Event !");
                let result = match c.unwrap() {
                    Event::Key(ke) => match ke {
                        Key::Char('q') =>
                        // self.game_over();
                        // return false;
                        {
                            Direction::None
                        }
                        Key::Up | Key::Char('w') | Key::Char(' ') => Direction::Up,
                        Key::Down | Key::Char('s') => Direction::Down,
                        Key::Right | Key::Char('d') => Direction::Right,
                        Key::Left | Key::Char('a') => Direction::Left,
                        _ => Direction::None,
                    },
                    Event::Mouse(me) => match me {
                        MouseEvent::Press(_, a, b) => {
                            let x = a as i16 - origin.x as i16 - 1;
                            let y = b as i16 - origin.y as i16 - 1;

                            Direction::To(Vector::new(x as f32, y as f32))
                        }
                        _ => Direction::None,
                    },
                    _ => Direction::None,
                };

                Self::run(&mut gen_index, player_id, result)
            }
        });

        loop {
            debug::write(&format!("LOOP"));
            let now = Instant::now();
            let dt = (now.duration_since(before).subsec_nanos() / 1_000_000) as u64;

            if dt < interval {
                thread::sleep(Duration::from_millis(interval - dt));
                continue;
            }

            before = now;

            // let my_receiver = &receiver;
            // my_receiver.recv().into_iter().for_each(|d| {
            //     // let result = d.get();
            //     debug::write(&format!("Iter results {:?}", d));
            //     self.run(d);
            // });

            if !self.update() {
                return;
            }
        }
    }

    fn run(gen_index: &mut GenIndexSync<EntitySync>, player_id: u16, direction: Direction) {
        let player_opt = gen_index.get(player_id as usize).unwrap();
        let mut player = unlock(&player_opt);

        match direction {
            Direction::To(end) => {
                gen_index.alloc_entity(wrap(Bullet::new(player.get_point(), end)));
            }
            Direction::None => (),
            _ => player.action(direction),
        }
    }

    pub fn update(&mut self) -> bool {
        let debug = false;

        for e in self.gen_index.clone() {
            renderer::clear(&e, self.stdout, self.origin, &self.map);
        }

        // clear
        for e in self.gen_index.clone() {
            let mut e_lock = unlock(&e);

            e_lock.update();
            e_lock.collide(&self.map);
        }

        let mut to_remove: Vec<usize> = Vec::new();

        for e in self.gen_index.clone() {
            let e_lock = unlock(&e);
            if e_lock.should_remove() {
                renderer::clear(&e, self.stdout, self.origin, &self.map);

                if let Some(id) = e_lock.get_id() {
                    to_remove.push(id);
                }
            }
        }

        for id in to_remove {
            debug::write(&format!("Remove ID {}", id));
            let remove = self.gen_index.free_index(id);
            if let Err(e) = remove {
                debug::write(&format!("Remove ID ERROR! {}", e));
            }
        }

        for e in self.gen_index.clone() {
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
