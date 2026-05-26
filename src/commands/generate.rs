use clap::{arg, Command};

pub fn generate_command() -> Command {
    let generate_readme_cmd = Command::new("readme")
        .about("create readme for a project")
        .arg(arg!(-f --force "Overwrite existing "))
        .arg(arg!(-b --backup "backup existing"))
        .arg(arg!(-p --path <PATH> "desired path"));

    let generate_gitignore_cmd = Command::new("git")
        .about("create a .gitignore file for a project")
        .arg(arg!(-f --force "Overwrite existing .gitignore file "))
        .arg(arg!(-b --backup "backup existing .gitignore file before overwrite"))
        .arg(arg!(-p --path <PATH> "desired path"));

    let generate_service_cmd = Command::new("service")
    .about("create a new backend service using convectional structure or as specified in the toolbox.toml file")
    .arg(arg!(-p --path <PATH> "desired path").required(true));

    Command::new("generate")
        .visible_aliases(["g", "-g"])
        .about("generate a new project or project files")
        .subcommand(generate_readme_cmd)
        .subcommand(generate_gitignore_cmd)
        .subcommand(generate_service_cmd)
        .arg(arg!(-p --path <PATH> "path"))
}
