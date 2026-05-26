use clap::{Arg, Command};

pub fn extension_command() -> Command {
    let create_cmd = Command::new("create")
        .about("create a new Lua extension")
        .aliases(["c", "-c"])
        .arg(
            Arg::new("name")
                .help("name of the extension")
                .required(true),
        );

    let update_cmd = Command::new("update")
        .about("open an extension in your preferred editor")
        .aliases(["u", "-u"])
        .arg(
            Arg::new("name")
                .help("name of the extension")
                .required(true),
        );

    let remove_cmd = Command::new("remove")
        .about("remove an extension")
        .aliases(["r", "rm"])
        .arg(
            Arg::new("name")
                .help("name of the extension")
                .required(true),
        );

    let bootstrap_cmd = Command::new("bootstrap")
        .about("import Lua/YAML extension pairs from a folder into the extensions directory")
        .aliases(["bs", "-bs"])
        .arg(
            Arg::new("path")
                .help("path to the folder containing extensions")
                .required(true),
        );

    let open_cmd = Command::new("open")
        .about("open the extensions directory in your preferred editor")
        .aliases(["o", "-o"]);

    Command::new("extension")
        .visible_aliases(["ext", "-ext"])
        .about("manage Lua extensions")
        .subcommand(create_cmd)
        .subcommand(update_cmd)
        .subcommand(remove_cmd)
        .subcommand(open_cmd)
        .subcommand(bootstrap_cmd)
}
