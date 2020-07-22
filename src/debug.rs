use crate::lazy_static;
use std::io::{stdin, stdout, Read, Write};
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Mutex;
use termion::{clear, color, cursor};

lazy_static! {
    static ref DEBUG_BUF: Mutex<Vec<std::string::String>> = Mutex::new(vec![]);
}

static MAX_DEBUG_LEN: usize = 20;

// pub fn setup(stdout: &mut impl Write) {
//     for l in 0..MAX_DEBUG_LEN {
//         writeln!(stdout, "{}\n", cursor::Goto(1, l as u16)).unwrap()
//     }
// }

pub fn write(stdout: &mut impl Write, output: &str) {
    let mut debug_buf = DEBUG_BUF.lock().unwrap();

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

// pub fn clear(stdout: &mut impl Write) {
//     let buf_len = DEBUG_BUF.lock().unwrap().len();
//
//     for l in 0..buf_len {
//         writeln!(
//             stdout,
//             "{goto}{clear}\r\n",
//             goto = cursor::Goto(1, l as u16),
//             clear = clear::UntilNewline
//         )
//         .unwrap()
//     }
// }
//
// pub fn clear(stdout: &mut impl Write) {
//     let line_no = DEBUG_LINE_NO.load(Ordering::SeqCst);
//     for l in 0..line_no {
//         writeln!(
//             stdout,
//             "{}{}",
//             cursor::Goto(1, l as u16),
//             clear::UntilNewline
//         )
//         .unwrap()
//     }
//     DEBUG_LINE_NO.store(1, Ordering::SeqCst);
// }
