extern crate unwort;
extern crate clap;

use clap::{Arg, ArgMatches, App, SubCommand};
use unwort::Dictionary;
use unwort::german::Dictionary as German;

pub fn string_to_dict(dict_name: &str) -> Option<Box<Dictionary>> {
	match dict_name.to_lowercase().as_str() {
		"german" => {
			match German::new("dict/german") {
				Ok(ger) => Some(Box::new(ger)),
				Err(e) => {
					println!("Could not create German dictionary: {}", e);
					None
				}
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

	let dictionary = matches.value_of("DICTIONARY").expect("Could not read dictionary variable");
	let dictionary = string_to_dict(dictionary).expect("Cannot continue without a valid dictionary");

	if let Some(m) = matches.subcommand_matches("add") {
		add(dictionary, m);
	}
}

fn add(mut dict: Box<Dictionary>, args: &ArgMatches) {
	for ref s in args.values_of("ENTRIES").expect("ENTRIES could not be read") {
		dict.add(&s);
	}
}
