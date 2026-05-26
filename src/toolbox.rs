use clap::ArgMatches;

use crate::helpers::console::LogMessage;
use crate::parsers::extension::parse_extension_options;
use crate::parsers::{
    app::{parse_uninstall_options, parse_upgrade_options},
    generator::parse_generator_options,
};
use crate::workers::gui::exec_gui;

pub async fn parse_commands(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("uninstall", sub_matches)) => parse_uninstall_options(sub_matches),
        Some(("upgrade", sub_matches)) => parse_upgrade_options(sub_matches),
        Some(("generate", sub_matches)) => parse_generator_options(sub_matches),
        Some(("extension", sub_matches)) => parse_extension_options(sub_matches),
        Some(("bookmarks", _)) => exec_gui().await,
        _ => {
            LogMessage::error("Invalid command");
            std::process::exit(1)
        }
    }
}
