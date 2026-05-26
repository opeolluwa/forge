use std::path::Path;

use clap::ArgMatches;

use crate::helpers::console::LogMessage;
use crate::workers::generator::GeneratorConfig;

/// Parses subcommands and delegates to the appropriate generator.
pub fn parse_generator_options(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("readme", args)) => {
            let config = GeneratorConfig::parse_options(args);
            if let Err(err) = GeneratorConfig::new(config.force, config.base_path, config.back_up)
                .generate_readme()
            {
                LogMessage::error(&format!("Failed to generate README: {}", err));
            } else {
                LogMessage::success("README.md generated successfully.");
            }
        }

        Some(("git", args)) => {
            let config = GeneratorConfig::parse_options(args);
            match GeneratorConfig::new(config.force, config.base_path, config.back_up)
                .generate_ignore_file()
            {
                Ok(_) => LogMessage::success(".gitignore file generated successfully."),
                Err(err) => LogMessage::error(&format!("Failed to generate .gitignore: {}", err)),
            }
        }

        Some(("service", args)) => {
            let base_path = args
                .get_one::<String>("path")
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| ".".to_string());

            GeneratorConfig::generate_service(&Path::new(&base_path).to_path_buf());
            LogMessage::info(&format!("Service generated at {}", base_path));
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
