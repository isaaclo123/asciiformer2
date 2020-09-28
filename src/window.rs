use std::io::{stdin, stdout, Stdin, StdinLock, Stdout};
use std::sync::{Arc, Mutex, MutexGuard};
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::terminal_size;

type Input = Stdin;
type Output = MouseTerminal<RawTerminal<Stdout>>;

pub struct Window {
    // stdin: Arc<Input>,
    // stdout: Arc<Mutex<Output>>,
    stdin: Input,
    stdout: Output,
    height: u16,
    width: u16,
}

impl Window {
    pub fn new() -> Self {
        let (width, height) = terminal_size().unwrap();

        Self {
            // stdin: Arc::new(stdin()),
            // stdout: Arc::new(Mutex::new(MouseTerminal::from(
            //     stdout().into_raw_mode().unwrap(),
            // ))),
            stdin: stdin(),
            stdout: MouseTerminal::from(stdout().into_raw_mode().unwrap()),
            width,
            height,
        }
    }

    pub fn get_stdin(&mut self) -> &mut Input {
        &mut self.stdin
    }

    pub fn get_stdout(&mut self) -> &mut Output {
        // Arc::get_mut(&mut self.stdout).unwrap().lock().unwrap()
        &mut self.stdout
    }
}
