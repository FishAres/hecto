use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode; // instant evaluation (no Enter required)

fn die(e: std::io::Error) {
    // error handling
    panic!(e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    //  you need to "keep this around", therefore binding
    // "unwrap" unwraps the object
    // from the "Result" or "Err" object
    // instead we'll do our own error handling
    // to not end up with half-drawn input

    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key {
                Key::Char(c) => {
                    // matches any character
                    // and binds iet to "c"
                    if c.is_control() {
                        println!("{:?}\r", c as u8); // print the byte (u8) version
                    } else {
                        println!("{:?} ({})\r", c as u8, c);
                    }
                }
                Key::Ctrl('q') => break,
                _ => println!("{:?}\r", key),
                // _ is for every case that hasn't been handled,
                // e.g. prints Up Down etc.
            },
            Err(err) => die(err),
        }
    }
}
