use std::{fs, path::Path};

use clap::ArgMatches;

use crate::{
    helpers::console::LogMessage,
    workers::scripts::{add_script_command, configure_scripts, execute_custom_script, list_script},
};

pub fn parse_script_options(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("configure", args)) => {
            let override_existing = *args.get_one::<bool>("overwrite").unwrap_or(&false);

            let _ = configure_scripts(override_existing);
        }

        Some(("add", args)) => {
            if let Some(script_file_path) = args
                .get_one::<String>("path")
                .map(|s| Path::new(s).to_path_buf())
            {
                let _ = add_script_command(&script_file_path);
            } else {
                LogMessage::error("invalid script path");
            }
        }

        Some(("remove", args)) => {
            if let Some(script) = args
                .get_one::<String>("name")
                .map(|script| format!("{script}.py"))
            //TODO: read the file extension
            {
                let _ = fs::remove_file(script);
            } else {
                LogMessage::error("script name is required");
            }
        }
        Some(("execute", args)) => {
            if let Some(script) = args
                .get_one::<String>("name")
                .map(|script| format!("{script}.py"))
            {
                let _ = execute_custom_script(&script);
            } else {
                LogMessage::error("script name is required");
            }
        }

        Some(("list", _)) => {
            list_script();
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
