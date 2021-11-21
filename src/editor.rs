use std::io::{self, stdout};
use std::default::Default;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
	pub fn run(&self) {
		let _stdout = stdout().into_raw_mode().unwrap();

		for key in io::stdin().keys() {
			match key {
				Ok(key) => match key {
					Key::Char(c) => 
						if c.is_control() {
							println!("{:?}\r", c as u8);
						} else {
							println!("{:?} ({})\r", c as u8, c);
						},
					Key::Ctrl('q') => break,
					_ => println!("{:?}\r", key),
				},
				Err(err) => die(err)
			}
		}
	}
}

impl Default for Editor {
	fn default() -> Self {
		Self {}
	}
}

fn die(e: io::Error) {
    panic!("{}", e);
}