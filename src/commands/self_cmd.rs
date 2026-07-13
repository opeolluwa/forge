use clap::Command;

pub fn self_command() -> Command {
    let self_uninstall = Command::new("uninstall").about("uninstall forge");

    let self_upgrade = Command::new("upgrade").about("upgrade forge");

    let self_configure = Command::new("configure").about("configure forge");

    Command::new("self")
        .about("manage and configure forge")
        .subcommand(self_uninstall)
        .subcommand(self_upgrade)
        .subcommand(self_configure)
}
