use super::{Diagnostic, Files, Reporter};
use codespan_reporting::diagnostic::{Label, LabelStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GlobalRootError {
	#[error("Advancement trigger must be `minecraft:tick`")]
	InvalidTrigger(String, String),
	#[error("Advancement icon must be `minecraft:knowledge_book`")]
	InvalidIcon(String, String),
	#[error("Invalid Title, It must be `Installed Datapacks`")]
	InvalidTitle(String, String),
	#[error("Description must be empty")]
	InvalidDescription(String, String),
	#[error("Invalid Background, It must be `gray_concrete.png`")]
	InvalidBackground(String, String),
	#[error("Invalid show_toast, It must be turn off")]
	InvalidShowToast(String, String),
	#[error("Invalid announce_to_chat, It must be turn off")]
	InvalidAnnounceToChat(String, String),
	#[error("There must be exactly one criteria")]
	InvalidCriteria(String),
}

fn quoted(input: &str) -> String {
	format!("\"{}\"", input)
}

use std::ops::Range;
impl GlobalRootError {
	fn keyword_searcher<'a>(
		&'a self,
		name: String,
		source: &'a str,
		keyword: &'a str,
		files: &mut Files<'a>,
	) -> (usize, Range<usize>) {
		let file_id = files.add(name, source);
		let predicate = &quoted(keyword);
		let start = source.find(predicate).unwrap();
		let end = start + predicate.len();
		let range = start..end;

		(file_id, range)
	}
}

impl Reporter for GlobalRootError {
	type Info = PathBuf;

	fn report(&self, info: Self::Info) -> Option<(Files, Diagnostic)> {
		let mut files = Files::new();
		let name = format!("{}", info.display());

		match self {
			Self::InvalidTrigger(source, trigger) => {
				let (file_id, range) = self.keyword_searcher(name, source, trigger, &mut files);
				let diagnostic = Diagnostic::error()
					.with_labels(vec![Label::new(LabelStyle::Primary, file_id, range)])
					.with_message("Invalid Trigger")
					.with_notes(vec!["Change it to `minecraft:tick`".to_owned()]);

				Some((files, diagnostic))
			}
			Self::InvalidIcon(source, icon) => {
				let (file_id, range) = self.keyword_searcher(name, source, icon, &mut files);
				let diagnostic = Diagnostic::error()
					.with_labels(vec![Label::new(LabelStyle::Primary, file_id, range)
						.with_message("Advancement Icon must be `minecraft:knowledge_book`")])
					.with_message("Invalid Icon")
					.with_notes(vec!["Change it to `minecraft:knowledge_book`".to_owned()]);
				Some((files, diagnostic))
			}
			Self::InvalidTitle(source, title) => {
				let (file_id, range) = self.keyword_searcher(name, source, title, &mut files);
				let diagnostic = Diagnostic::error()
					.with_labels(vec![Label::new(LabelStyle::Primary, file_id, range)
						.with_message("Advancement Title must be `Installed Datapacks`")])
					.with_message("Invalid Title")
					.with_notes(vec!["Change it to `Installed Datapacks`".to_owned()]);

				Some((files, diagnostic))
			}
			Self::InvalidDescription(source, description) => {
				let (file_id, range) = self.keyword_searcher(name, source, description, &mut files);
				let diagnostic = Diagnostic::error()
					.with_labels(vec![Label::new(LabelStyle::Primary, file_id, range)
						.with_message("Advancement Description must be empty")])
					.with_message("Invalid Description")
					.with_notes(vec!["Change it to \"\"".to_owned()]);

				Some((files, diagnostic))
			}
			Self::InvalidBackground(source, background) => {
				let (file_id, range) = self.keyword_searcher(name, source, background, &mut files);
				let diagnostic = Diagnostic::error()
					.with_labels(vec![Label::new(LabelStyle::Primary, file_id, range)
						.with_message("Advancement Background must be `gray_concrete.png`")])
					.with_message("Invalid Background")
					.with_notes(vec![
						"Change it to `minecraft:textures/block/gray_concrete.png`".to_owned(),
					]);

				Some((files, diagnostic))
			}
			Self::InvalidShowToast(source, show_toast) => {
				let (file_id, range) = self.keyword_searcher(name, source, show_toast, &mut files);
				let diagnostic = Diagnostic::error()
					.with_labels(vec![Label::new(LabelStyle::Primary, file_id, range)
						.with_message("`show_toast` should be `false`")])
					.with_message("Invalid `show_toast`")
					.with_notes(vec!["Change it to `false`".to_owned()]);

				Some((files, diagnostic))
			}
			Self::InvalidAnnounceToChat(source, announce_to_chat) => {
				let (file_id, range) =
					self.keyword_searcher(name, source, announce_to_chat, &mut files);
				let diagnostic = Diagnostic::error()
					.with_labels(vec![Label::new(LabelStyle::Primary, file_id, range)
						.with_message("`announce_to_chat` should be `false`")])
					.with_message("Invalid `announce_to_chat`")
					.with_notes(vec!["Change it to `false`".to_owned()]);

				Some((files, diagnostic))
			}
			Self::InvalidCriteria(source) => {
				let (file_id, range) = self.keyword_searcher(name, source, "criteria", &mut files);
				let diagnostic = Diagnostic::error()
					.with_labels(vec![Label::new(LabelStyle::Primary, file_id, range)])
					.with_message("Too many criteria")
					.with_notes(vec!["There must be exactly one criteria".to_owned()]);

				Some((files, diagnostic))
			}
		}
	}
}

#[derive(Deserialize, Serialize)]
pub struct GlobalRoot {
	display: Display,
	criteria: HashMap<String, Criteria>,
}

impl GlobalRoot {
	pub fn verify(&self, source: &str) -> Result<(), GlobalRootError> {
		self.display.verify(source)?;
		self.criteria
			.iter()
			.try_for_each(|(_, criteria)| criteria.verify(source))?;

		if self.criteria.len() != 1 {
			return Err(GlobalRootError::InvalidCriteria(source.to_owned()));
		}

		Ok(())
	}
}

#[derive(Deserialize, Serialize)]
struct Display {
	title: String,
	description: String,
	icon: Icon,
	background: String,
	show_toast: bool,
	announce_to_chat: bool,
}

impl Display {
	fn verify(&self, source: &str) -> Result<(), GlobalRootError> {
		if self.title != "Installed Datapacks" {
			return Err(GlobalRootError::InvalidTitle(
				source.to_owned(),
				"title".to_owned(),
			));
		}
		if !self.description.is_empty() {
			return Err(GlobalRootError::InvalidDescription(
				source.to_owned(),
				"description".to_owned(),
			));
		}
		if self.background != "minecraft:textures/block/gray_concrete.png" {
			return Err(GlobalRootError::InvalidBackground(
				source.to_owned(),
				"background".to_owned(),
			));
		}
		if self.show_toast {
			return Err(GlobalRootError::InvalidShowToast(
				source.to_owned(),
				"show_toast".to_owned(),
			));
		}
		if self.announce_to_chat {
			return Err(GlobalRootError::InvalidAnnounceToChat(
				source.to_owned(),
				"announce_to_chat".to_owned(),
			));
		}

		self.icon.verify(source)?;
		Ok(())
	}
}

#[derive(Deserialize, Serialize)]
struct Icon {
	item: String,
}

impl Icon {
	fn verify(&self, source: &str) -> Result<(), GlobalRootError> {
		if self.item != "minecraft:knowledge_book" {
			return Err(GlobalRootError::InvalidIcon(
				source.to_owned(),
				"item".to_owned(),
			));
		}

		Ok(())
	}
}

#[derive(Deserialize, Serialize)]
struct Criteria {
	trigger: String,
}

impl Criteria {
	fn verify(&self, source: &str) -> Result<(), GlobalRootError> {
		if self.trigger != "minecraft:tick" {
			return Err(GlobalRootError::InvalidTrigger(
				source.to_owned(),
				self.trigger.to_owned(),
			));
		}

		Ok(())
	}
}
