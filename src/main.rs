use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;

                if c.is_control() {
                    println!("{:#b}\r", b);
                } else {
                    println!("{:#b} ({})\r", b, c);
                }

                println!("{}", c);
                if b == to_control_byte('q') {
                    break;
                }
            },
            Err(err) => die(err)
        }
    }
}

fn to_control_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: io::Error) {
    panic!("{}", e);
}
