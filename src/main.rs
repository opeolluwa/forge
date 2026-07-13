use clap::Command;
use lib_forge::{
    commands::{
        bookmarks::bookmarks_command, extension::extension_command, generate::generate_command,
        self_cmd::self_command,
    },
    errors::app::AppError,
    forge::parse_commands,
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let matches = Command::new("forge")
        .display_name("Forge")
        .about("lightweight extensible, command line toolchain for software builders")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(self_command())
        .subcommand(bookmarks_command())
        .subcommand(generate_command())
        .subcommand(extension_command())
        .get_matches();

    parse_commands(matches).await;

    Ok(())
}
