use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode; // instant evaluation (no Enter required)

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111 // biwise and
}

fn die(e: std::io::Error) {
    // error handling
    panic!(e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    //  you need to "keep this around", therefore binding

    for b in io::stdin().bytes() {
        // "unwrap" unwraps the object
        // from the "Result" or "Err" object
        // instead we'll do our own error handling
        // to not end up with half-drawn input

        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    println!("{:?} ({}\r", b, c);
                }
                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(err) => die(err),
        }
    }
}
