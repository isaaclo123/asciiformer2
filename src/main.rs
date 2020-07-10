#[macro_use]
extern crate lazy_static;

extern crate termion;

mod consts;
mod game;
mod map;

use game::Game;
use std::io::stdout;
use termion::async_stdin;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

fn main() {
    let stdin = async_stdin().events();
    let stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    let mut game = Game::new(stdin, stdout, "map1.txt");
    game.start();
}
