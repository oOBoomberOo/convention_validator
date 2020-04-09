pub mod datapack_advancement;
use std::path::PathBuf;

pub trait Convention {
	type Verify: Reporter;
	type Error;
	fn verify(root: &PathBuf) -> Result<Self::Verify, Self::Error>;
	
	fn strip(root: &PathBuf, path: PathBuf) -> PathBuf {
		match path.strip_prefix(root) {
			Ok(path) => path.to_owned(),
			Err(e) => panic!("Failed to stripprefix: {}", e)
		}
	}
}

use codespan_reporting::diagnostic::Diagnostic as OriginalDiagnostic;
use codespan_reporting::files::SimpleFiles;

pub type Files<'a> = SimpleFiles<String, &'a str>;
pub type Diagnostic = OriginalDiagnostic<usize>;

pub trait Reporter {
	type Info;

	fn report(&self, info: Self::Info) -> Option<(Files, Diagnostic)>;
}

use std::io;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum ConventionError {
	#[error("I/O error: {0}")]
	Io(io::Error),
	#[error("Serde: {0}")]
	Serde(serde_json::Error)
}

impl From<io::Error> for ConventionError {
	fn from(error: io::Error) -> ConventionError {
		ConventionError::Io(error)
	}
}

impl From<serde_json::Error> for ConventionError {
	fn from(error: serde_json::Error) -> ConventionError {
		ConventionError::Serde(error)
	}
}

pub mod prelude {
	pub use super::{Convention, ConventionError, Files, Diagnostic, Reporter};
	pub use super::datapack_advancement::DatapackAdvancement;
}