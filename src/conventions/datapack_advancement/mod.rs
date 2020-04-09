use super::prelude::*;
use std::path::PathBuf;
use serde_json as js;
use std::fs;

mod global_root;

pub struct DatapackAdvancement;

impl DatapackAdvancement {
	fn global_root(root: &PathBuf) -> PathBuf {
		root.join("data").join("global").join("advancements").join("root.json")
	}
}

impl Convention for DatapackAdvancement {
	type Verify = Verify;
	type Error = ConventionError;

	fn verify(root: &PathBuf) -> Result<Self::Verify, Self::Error> {
		let global_root = Self::global_root(root);
		
		if !global_root.exists() {
			return Ok(Verify::GlobalRootNotFound);
		}

		let source = fs::read_to_string(&global_root)?;
		let content: global_root::GlobalRoot = js::from_str(&source)?;

		if let Err(format_error) = content.verify(&source) {
			return Ok(Verify::InvalidRootFormat(global_root, format_error));
		}

		Ok(Verify::Ok)
	}
}

use thiserror::Error;
#[derive(Debug, Error)]
pub enum Verify {
	#[error("[MC001] Cannot find global root advancement")]
	GlobalRootNotFound,

	#[error("[MC002] Invalid format: {0}")]
	InvalidRootFormat(PathBuf, global_root::GlobalRootError),

	#[error("[MC000] This datapack is following Datapack Advancement convention correctly!")]
	Ok,
	#[error("Do nothing...")]
	Nothing
}

impl Reporter for Verify {
	type Info = ();

	fn report(&self, _: Self::Info) -> Option<(Files, Diagnostic)> {
		match self {
			Self::GlobalRootNotFound => {
				let files = Files::new();
				let diagnostic = Diagnostic::error()
					.with_code("MC001")
					.with_message("Cannot find global root json for `Datapack Advancement` convention")
					.with_notes(vec![
						"Cannot find `data/global/advancments/root.json`".to_owned(),
					]);

				Some((files, diagnostic))
			},
			Self::InvalidRootFormat(path, error) => error.report(path.to_owned()),
			Self::Ok => {
				let files = Files::new();
				let diagnostic = Diagnostic::note().with_code("MC000").with_message("This datapack is following `Datapack Advancement` correctly!");
				Some((files, diagnostic))
			},
			Self::Nothing => None
		}
	}
}