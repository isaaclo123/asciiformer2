use crate::consts::{EntityType, TEXTURE_MAP};
use crate::map::Map;
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};
use termion::event::*;

use termion::event::Key;
use termion::input::{Events, TermRead};
use termion::AsyncReader;
use termion::{clear, color, cursor, terminal_size};

pub struct Game<W> {
    width: u16,
    height: u16,
    stdin: Events<AsyncReader>,
    stdout: W,
    map: Map,
}

impl<W: Write> Game<W> {
    pub fn new(stdin: Events<AsyncReader>, stdout: W, map_file: impl AsRef<Path>) -> Game<W> {
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
        let interval = 10;

        write!(self.stdout, "{}", cursor::Hide).unwrap();

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

    pub fn update(&mut self) -> bool {
        if let Some(c) = self.stdin.next() {
            // write!(self.stdout, "test").unwrap();
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

        for row in 0..self.map.height {
            for col in 0..self.map.width {
                let texture = match TEXTURE_MAP.get(self.map.data[row as usize][col as usize]) {
                    Some(e) => *e,
                    // TODO should throw error if character not found
                    None => ' ',
                };
                write!(
                    self.stdout,
                    "{goto}{texture}",
                    // add 1 as cursor is 1-indexed
                    goto = cursor::Goto(offset_width + col + 1, offset_height + row + 1),
                    texture = texture
                )
                .unwrap();
            }
        }

        self.stdout.flush().unwrap();
    }
}
