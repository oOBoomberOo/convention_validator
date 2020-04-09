use structopt::StructOpt;
use std::path::PathBuf;
use codespan_reporting::term::ColorArg;

#[derive(StructOpt)]
struct Opt {
	#[structopt(short, long)]
	watch: bool,

	#[structopt(parse(from_os_str))]
	path: PathBuf,

	#[structopt(short, long, default_value = "auto", possible_values = ColorArg::VARIANTS)]
	color: ColorArg,
}

mod verify_convention;
mod validator;
mod program_error;
mod conventions;
use program_error::ProgramError;
use validator::Validator;

fn main() {
	let option = Opt::from_args();

	if let Err(error) = run(option) {
		println!("{}", error);
	}
}

fn run(option: Opt) -> Result<(), ProgramError> {
	let validator = Validator::from(option);
	validator.run();

	Ok(())
}