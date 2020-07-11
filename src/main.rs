#[macro_use]
extern crate lazy_static;

extern crate termion;

mod consts;
mod entities;
mod game;
mod map;
mod textures;
mod vectors;

use game::Game;
use std::io::stdout;
use termion::async_stdin;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

fn main() {
    let stdin = &mut async_stdin();
    let stdout = &mut MouseTerminal::from(stdout().into_raw_mode().unwrap());

    let mut game = Game::new(stdin, stdout, "map1.txt");
    game.start();
}
