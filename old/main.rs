#[macro_use]
extern crate lazy_static;
extern crate termion;

mod consts;
mod debug;
mod entities;
mod game;
mod genindex;
mod helpers;
mod map;
mod renderer;
mod textures;
mod vectors;

use game::Game;
use std::io::{stdin, stdout, Stdin, Stdout};
use termion::async_stdin;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;

fn main() {
    // let stdin = &mut async_stdin();
    // let stdin = &mut async_stdin();
    let mut stdin = &mut stdin();
    let mut stdout = &mut MouseTerminal::from(stdout().into_raw_mode().unwrap());
    // other_debug::setup(stdout);

    // static game: Game<Stdin, Stdout> = Game::new(stdin, stdout, "map1.txt");
    let mut game = Game::new(stdin, stdout, "map1.txt");
    game.start();
}
