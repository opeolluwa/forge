use clap::ArgMatches;

use std::path::Path;

use crate::{
    helpers::console::LogMessage,
    workers::extension::{
        bootstrap_extensions, create_extension, open_extensions_dir, remove_extension,
        update_extension,
    },
};

pub fn parse_extension_options(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("create", args)) => {
            let name = args.get_one::<String>("name").expect("name is required");
            let _ = create_extension(name);
        }

        Some(("update", args)) => {
            let name = args.get_one::<String>("name").expect("name is required");
            let _ = update_extension(name);
        }

        Some(("remove", args)) => {
            let name = args.get_one::<String>("name").expect("name is required");
            let _ = remove_extension(name);
        }

        Some(("bootstrap", args)) => {
            let path = args.get_one::<String>("path").expect("path is required");
            let _ = bootstrap_extensions(Path::new(path));
        }

        Some(("open", _)) => {
            let _ = open_extensions_dir();
        }

        Some((other, _)) => {
            LogMessage::warning(&format!("Unknown subcommand '{}'", other));
            std::process::exit(1);
        }

        None => {
            LogMessage::error("No subcommand provided. Use `--help` to see available options.");
            std::process::exit(1);
        }
    }
}
