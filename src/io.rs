use lazy_static::lazy_static;
use std::io::{stdin, stdout, Stdin, StdinLock, Stdout};
use std::sync::{Arc, Mutex, MutexGuard};
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};

pub type Input = Stdin;
pub type Output = MouseTerminal<RawTerminal<Stdout>>;

lazy_static! {
    pub static ref STDIN: Stdin = stdin();
    // pub static ref STDOUT: Arc<Mutex<Output>> = Arc::new(Mutex::new(MouseTerminal::from(
    //     stdout().into_raw_mode().unwrap()
    // )));
    pub static ref STDOUT: Arc<Mutex<Output>> = Arc::new(Mutex::new(MouseTerminal::from(
        stdout().into_raw_mode().unwrap()
    )));
}

pub fn get_stdout<'a>() -> MutexGuard<'a, Output> {
    STDOUT.lock().unwrap()
}
