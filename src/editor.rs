use std::io::{self, stdout, Write};
use std::default::Default;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
	should_quit: bool,
}

impl Editor {
	pub fn run(&mut self) {
		let _stdout = stdout().into_raw_mode().unwrap();

		loop {
			if let Err(error) = self.refresh_screen() {
				die(error);
			}

			if self.should_quit {
				break;
			}

			if let Err(error) = self.process_keypress() {
				die(error);
			}
		}
	}

	fn refresh_screen(&self) -> Result<(), io::Error> {
		print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

		if self.should_quit {
			println!("Goodbye...\r");
		} else {
			self.draw_rows();
			print!("{}", termion::cursor::Goto(1, 1));
		}

		stdout().flush()
	}

	fn process_keypress(&mut self) -> Result<(), io::Error> {
		let pressed_key = read_key()?;

		match pressed_key {
			Key::Ctrl('q') => self.should_quit = true,
			_ => ()
		}

		Ok(())
	}

	fn draw_rows(&self) {
		for _ in 0..24 {
			println!("~\r");
		}
	}
}

impl Default for Editor {
	fn default() -> Self {
		Self {
			should_quit: false,
		}
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
	print!("{}", termion::clear::All);
    panic!("{}", e);
}