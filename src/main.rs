extern crate unwort;
extern crate clap;

use clap::{Arg, App, SubCommand};
use unwort::Dictionary;
use unwort::german::Dictionary as German;

pub fn string_to_dict(dict_name: &str) -> Option<Box<Dictionary>> {
	match dict_name.to_lowercase().as_str() {
		"german" => {
			if let Ok(ger) = German::new("dict/german") {
				Some(Box::new(ger))
			} else {
				None
			}
		},
		other => {
			println!("Dictionary {} does not match any known dictionary.", other);
			None
		}
	}
}

fn main() {
	let mut app = App::new("Unwort CLI").version(env!("CARGO_PKG_VERSION")).author("Arne Dußin")
		.about("Command line front-end for the unwort library.");
	app = app.arg(Arg::with_name("DICTIONARY")
		.required(true)
		.help("The dictionaries name the command should be executed on")
		.index(1)
	);
	app = app.subcommand(SubCommand::with_name("add").about("Adds the given words to the dictionary")
		.arg(Arg::with_name("ENTRIES")
			 .multiple(true)
			 .required(true)
			 .help("The list of entries to add")
			 .index(1)
		)
	);
	let matches = app.get_matches();

	let dictionary = matches.value_of("DICTIONARY").unwrap();
	let dictionary = string_to_dict(dictionary).unwrap();
}
