use std::io::{self, stdout};
use std::default::Default;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
	pub fn run(&self) {
		let _stdout = stdout().into_raw_mode().unwrap();

		loop {
			if let Err(error) = self.process_keypress() {
				die(error);
			}
		}
	}

	fn process_keypress(&self) -> Result<(), io::Error> {
		let pressed_key = read_key()?;

		match pressed_key {
			Key::Ctrl('q') => panic!("Program end"),
			_ => ()
		}

		Ok(())
	}
}

impl Default for Editor {
	fn default() -> Self {
		Self {}
	}
}

fn read_key() -> Result<Key, io::Error> {
	loop {
		if let Some(key) = io::stdin().lock().keys().next() {
			return key;
		}
	}
}

fn die(e: io::Error) {
    panic!("{}", e);
}