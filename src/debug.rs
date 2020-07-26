use std::io::{stdout, Stdout, Write};
use std::sync::Mutex;
use termion::cursor;

lazy_static! {
    static ref DEBUG_BUF: Mutex<Vec<std::string::String>> = Mutex::new(vec![]);
    static ref DEBUG_STDOUT: Mutex<Stdout> = Mutex::new(stdout());
}

static MAX_DEBUG_LEN: usize = 30;
static DEBUG: bool = true;

pub fn write(output: &str) {
    if !DEBUG {
        return;
    }

    let mut debug_buf = DEBUG_BUF.lock().unwrap();
    let mut stdout = DEBUG_STDOUT.lock().unwrap();

    for (i, line) in debug_buf.iter().enumerate() {
        for j in 0..line.len() {
            write!(
                stdout,
                "{goto} ",
                goto = cursor::Goto(j as u16 + 1, i as u16 + 1),
            )
            .unwrap()
        }
    }

    if debug_buf.len() > MAX_DEBUG_LEN - 1 {
        debug_buf.remove(0);
    }
    debug_buf.push(output.to_owned());

    for (i, line) in debug_buf.iter().enumerate() {
        writeln!(
            stdout,
            "{goto}{line}",
            goto = cursor::Goto(1, i as u16 + 1),
            line = line
        )
        .unwrap()
    }
}
