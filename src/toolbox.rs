use clap::ArgMatches;

use crate::helpers::console::LogMessage;
use crate::parsers::extension::parse_extension_options;
use crate::parsers::{
    app::{parse_uninstall_options, parse_upgrade_options},
    generator::parse_generator_options,
};
use crate::workers::gui::exec_gui;
use crate::workers::lua_runner::{run_extension, show_extension_help};

pub async fn parse_commands(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("uninstall", sub_matches)) => parse_uninstall_options(sub_matches),
        Some(("upgrade", sub_matches)) => parse_upgrade_options(sub_matches),
        Some(("generate", sub_matches)) => parse_generator_options(sub_matches),
        Some(("extension", sub_matches)) => parse_extension_options(sub_matches),
        Some(("bookmarks", _)) => exec_gui().await,
        Some((name, _)) => {
            if let Some(base) = name.strip_suffix("-help") {
                if let Err(e) = show_extension_help(base) {
                    LogMessage::error(&format!("no match: {}", e));
                    std::process::exit(1);
                }
            } else if let Err(e) = run_extension(name) {
                LogMessage::error(&format!("no match: {}", e));
                std::process::exit(1);
            }
        }
        None => {
            LogMessage::error("no command provided");
            std::process::exit(1);
        }
    }
}
