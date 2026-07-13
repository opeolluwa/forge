use clap::Command;

pub fn bookmarks_command() -> Command {
    let gui_cmd = Command::new("ui").about("open a GUI to manage bookmarks");

    Command::new("bookmarks")
        .about("manage and configure forge")
        .subcommand(gui_cmd)
}
