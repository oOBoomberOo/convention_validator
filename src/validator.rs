use super::Opt;
use crate::conventions::prelude::*;
use crate::verify_convention::VerifyConvention;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::StandardStream;
use codespan_reporting::term::ColorArg;
use codespan_reporting::term::Config;
use std::path::PathBuf;
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc;
use std::time::Duration;
use std::fs;

pub struct Validator {
	watch: bool,
	path: PathBuf,
	color: ColorArg,
}

impl Validator {
	pub fn run(&self) {
		if self.watch {
			if let Err(e) = self.watcher() {
				println!("{}", e);
			}
		} else {
			self.once();
		}
	}

	fn watcher(&self) -> notify::Result<()> {
		let (tx, rx) = mpsc::channel();
		
		let mut watcher = watcher(tx, Duration::from_secs(1))?;
		watcher.watch(&self.path, RecursiveMode::Recursive)?;

		self.once();

		loop {
			match rx.recv() {
				Ok(event) => self.handle_event(event),
				Err(e) => println!("{}", e)
			}
		}
	}

	fn handle_event(&self, event: DebouncedEvent) {
		match event {
			DebouncedEvent::Create(_)
			| DebouncedEvent::Remove(_)
			| DebouncedEvent::Rename(_, _)
			| DebouncedEvent::Write(_) => self.once(),
			_ => ()
		}
	}

	fn once(&self) {
		if let Err(e) = self.process() {
			println!("{}", e);
		}
	}

	fn process(&self) -> Result<(), ConventionError> {
		let diagnostics = vec![
			DatapackAdvancement::verify(&self.path)?.into()
		];
		self.report(diagnostics)
	}

	fn report(&self, diagnostics: Vec<VerifyConvention>) -> Result<(), ConventionError> {
		let mut writer = StandardStream::stdout(self.color.into());
		let config = Config::default();

		for diagnostic in diagnostics {
			if let Some((files, diagnostic)) = diagnostic.report(()) {
				term::emit(&mut writer, &config, &files, &diagnostic)?;
			}
		}

		Ok(())
	}
}

impl From<Opt> for Validator {
	fn from(opt: Opt) -> Validator {
		let path = fs::canonicalize(opt.path).unwrap();

		Validator {
			watch: opt.watch,
			path,
			color: opt.color,
		}
	}
}
