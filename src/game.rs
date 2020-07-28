use crate::debug;
use crate::entities::{Bullet, Direction, Entity, EntitySync, Player};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use termion::async_stdin;

use crate::genindex::GenIndex;
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
    gen_index: GenIndex<EntitySync>,
}

impl<'a, R: Read + Send, W: Write + Send> Game<'a, R, W> {
    pub fn new(stdin: &'a mut R, stdout: &'a mut W, map_file: impl AsRef<Path>) -> Self {
        let (width, height) = terminal_size().expect("Failed to get Terminal Size");
        let map = Map::load_from_file(map_file).unwrap();

        let offset_width = (width - map.width) / 2;
        let offset_height = (height - map.height) / 2;

        let mut gen_index: GenIndex<EntitySync> = GenIndex::new(100);

        let player = Arc::new(Mutex::new(Player::new(20.1, 20.7, "My Name")));

        let player_id = gen_index.alloc_index(player).unwrap();
        gen_index
            .get(player_id)
            .unwrap()
            .lock()
            .unwrap()
            .set_id(player_id);

        Game {
            width: width,
            height: height,
            map: Arc::new(Mutex::new(map)),
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
            gen_index,
            // player: RefCell::new(Player::new(20.1, 20.7, "My Name")),
        }
    }

    pub fn start(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();

        self.map.lock().unwrap().draw_map(self.stdout, self.origin);

        let mut before = Instant::now();
        let interval = 60;

        // let (sender, receiver) = channel();

        // let stdin = self.stdin;
        let origin = self.origin;
        let stdin = Arc::new(stdin());

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
                // sender.send(result).unwrap();

                self.run(result)
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

            self.clear();

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

    fn run(&mut self, direction: Direction) {
        let player_opt = self.gen_index.get(self.player_id as usize).unwrap();
        let mut player = player_opt.lock().unwrap();

        match direction {
            Direction::To(end) => {
                let bullet_id = self
                    .gen_index
                    .alloc_index(Arc::new(Mutex::new(Bullet::new(player.get_point(), end))))
                    .unwrap();

                self.gen_index
                    .get(bullet_id)
                    .unwrap()
                    .lock()
                    .unwrap()
                    .set_id(bullet_id);
            }
            Direction::None => (),
            _ => player.action(direction),
        }
    }

    pub fn clear(&mut self) {
        for e in &mut self.gen_index {
            renderer::clear(&e, self.stdout, self.origin, &self.map);
        }
    }

    pub fn update(&mut self) -> bool {
        let debug = false;

        // Clear

        // if let Some(c) = self.stdin.events().next() {
        //     match c.unwrap() {
        //         Event::Key(ke) => match ke {
        //             Key::Char('q') => {
        //                 // self.game_over();
        //                 return false;
        //             }
        //             Key::Up | Key::Char('w') | Key::Char(' ') => self.run(Direction::Up),
        //             Key::Down | Key::Char('s') => self.run(Direction::Down),
        //             Key::Right | Key::Char('d') => self.run(Direction::Right),
        //             Key::Left | Key::Char('a') => self.run(Direction::Left),
        //             _ => {
        //                 if debug {
        //                     self.run(Direction::None)
        //                 }
        //             }
        //         },
        //         Event::Mouse(me) => match me {
        //             MouseEvent::Press(_, a, b) => {
        //                 let x = a as i16 - self.origin.x as i16 - 1;
        //                 let y = b as i16 - self.origin.y as i16 - 1;

        //                 // let sym = if self.map.borrow().get(x, y).is_some() {
        //                 //     'W'
        //                 // } else {
        //                 //     ' '
        //                 // };

        //                 self.run(Direction::To(Vector::new(x as f32, y as f32)));
        //             }
        //             _ => (), // MouseEvent::Release(a, b) | MouseEvent::Hold(a, b) => {
        //                      //     // write!(self.stdout, "{}x", cursor::Goto(a, b)).unwrap();
        //                      // }
        //         },
        //         _ => self.run(Direction::None),
        //     }
        // } else {
        //     if !debug {
        //         self.run(Direction::None)
        //     }
        // }

        for e in &mut self.gen_index {
            let e_lock = e.lock().unwrap();
            e_lock.update();
            e_lock.collide(&self.map);
        }

        let mut to_remove: Vec<usize> = Vec::new();

        for e in &mut self.gen_index {
            let e_lock = e.lock().unwrap();
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

        for e in &mut self.gen_index {
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
