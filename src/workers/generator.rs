use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::ArgMatches;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};
use serde::{Deserialize, Serialize};

use crate::constants::TEMPLATES_DIR;
use crate::errors::file_system::FileSystemError;
use crate::helpers::{console::LogMessage, file_system::file_exists_in_path};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorConfig {
    pub force: bool,
    pub base_path: PathBuf,
    pub back_up: bool,
}

impl GeneratorConfig {
    pub fn new(force: bool, base_path: PathBuf, back_up: bool) -> Self {
        Self {
            force,
            base_path,
            back_up,
        }
    }

    pub fn parse_options(command_arguments: &ArgMatches) -> Self {
        let force = *command_arguments.get_one::<bool>("force").unwrap_or(&false);
        let back_up = *command_arguments
            .get_one::<bool>("backup")
            .unwrap_or(&false);
        let base_path = command_arguments
            .get_one::<String>("path")
            .map(|s| Path::new(s).to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        Self {
            force,
            base_path,
            back_up,
        }
    }

    pub fn generate_readme(&self) -> Result<(), FileSystemError> {
        let file_exists = file_exists_in_path(self.base_path.as_path(), "README.md");
        let file_path = self.base_path.join("README.md");
        let backup_path = self.base_path.join("README.md.bak");

        // Case 1: File exists but overwriting not allowed
        if file_exists && !self.force {
            return Err(FileSystemError::OperationError(
                "README.md already exists — use --force to overwrite".to_string(),
            ));
        }

        // Case 2: File exists and overwrite is allowed
        if file_exists && self.force {
            if self.back_up {
                if let Err(error) = fs::copy(&file_path, &backup_path) {
                    return Err(FileSystemError::OperationError(error.to_string()));
                }
                LogMessage::info(&format!("Backup created at {:?}", backup_path));
            } else {
                let confirm = Confirm::new()
                    .with_prompt("The current README.md will not be backed up. Continue?")
                    .default(false)
                    .interact()
                    .unwrap_or(false);

                if !confirm {
                    LogMessage::info("Operation cancelled by user.");
                    return Ok(());
                }
            }

            fs::remove_file(&file_path).map_err(|err| {
                FileSystemError::OperationError(format!("Failed to remove file: {}", err))
            })?;
            LogMessage::info("Old README.md removed.");
        }

        // Case 3: Create new README template
        Self::create_readme_template(&file_path)?;
        Ok(())
    }

    pub fn generate_ignore_file(&self) -> Result<(), FileSystemError> {
        let file_exists = file_exists_in_path(self.base_path.as_path(), ".gitignore");
        let file_path = self.base_path.join(".gitignore");
        let backup_path = self.base_path.join(".gitignore.bak");

        if file_exists && !self.force {
            LogMessage::warning(
                "A .gitignore file already exists in the selected path. Use the '--force' flag to overwrite it.",
            );
            return Err(FileSystemError::OperationError(
                ".gitignore already exists — use --force to overwrite".to_string(),
            ));
        }

        if file_exists && self.force {
            if self.back_up {
                fs::copy(&file_path, &backup_path).map_err(|err| {
                    FileSystemError::OperationError(format!("Backup failed: {}", err))
                })?;
                LogMessage::info(&format!("Backup created at {:?}", backup_path));
            }

            fs::remove_file(&file_path).map_err(|err| {
                FileSystemError::OperationError(format!("Failed to remove file: {}", err))
            })?;
            LogMessage::info("Old .gitignore removed.");
        }

        Self::create_git_ignore_template(&file_path)?;
        Ok(())
    }

    pub fn generate_service(base_path: &PathBuf) {
        let folders = [
            "config",
            "controllers",
            "services",
            "entities",
            "dto",
            "src",
        ];

        if !Path::new(base_path).exists() {
            if let Err(err) = fs::create_dir(base_path) {
                LogMessage::error(&format!("Failed to create base directory: {}", err));
                return;
            }
        }

        LogMessage::success(&format!(
            "Creating new service at {:?}",
            base_path
                .canonicalize()
                .unwrap_or_else(|_| base_path.clone())
        ));

        for dir in folders {
            let path = Path::new(base_path).join(dir);
            if let Err(err) = fs::create_dir(&path) {
                LogMessage::warning(&format!("Could not create folder {:?}: {}", path, err));
            }
        }
    }

    fn create_git_ignore_template(full_path: &Path) -> Result<(), FileSystemError> {
        let supported_technologies = vec![
            "AL",
            "Actionscript",
            "Ada",
            "Agda",
            "Android",
            "AppEngine",
            "AppceleratorTitanium",
            "ArchLinuxPackages",
            "Autotools",
            "C++",
            "C",
            "CFWheels",
            "CMake",
            "CUDA",
            "CakePHP",
            "ChefCookbook",
            "Clojure",
            "CodeIgniter",
            "CommonLisp",
            "Composer",
            "Concrete5",
            "Coq",
            "CraftCMS",
            "D",
            "DM",
            "Dart",
            "Delphi",
            "Drupal",
            "EPiServer",
            "Eagle",
            "Elisp",
            "Elixir",
            "Elm",
            "Erlang",
            "ExpressionEngine",
            "ExtJs",
            "Fancy",
            "Finale",
            "FlaxEngine",
            "ForceDotCom",
            "Fortran",
            "FuelPHP",
            "GWT",
            "Gcov",
            "GitBook",
            "Go",
            "Godot",
            "Gradle",
            "Grails",
            "Haskell",
            "IGORPro",
            "Idris",
            "JBoss",
            "JENKINS_HOME",
            "Java",
            "Jekyll",
            "Joomla",
            "Julia",
            "KiCad",
            "Kohana",
            "Kotlin",
            "LICENSE",
            "LabVIEW",
            "Laravel",
            "Leiningen",
            "LemonStand",
            "Lilypond",
            "Lithium",
            "Lua",
            "Magento",
            "Maven",
            "Mercury",
            "MetaProgrammingSystem",
            "Nanoc",
            "Nim",
            "Node",
            "OCaml",
            "Objective-C",
            "Opa",
            "OpenCart",
            "OracleForms",
            "Packer",
            "Perl",
            "Phalcon",
            "PlayFramework",
            "Plone",
            "Prestashop",
            "Processing",
            "PureScript",
            "Python",
            "Qooxdoo",
            "Qt",
            "R",
            "ROS",
            "Racket",
            "Rails",
            "Raku",
            "RhodesRhomobile",
            "Ruby",
            "Rust",
            "SCons",
            "Sass",
            "Scala",
            "Scheme",
            "Scrivener",
            "Sdcc",
            "SeamGen",
            "SketchUp",
            "Smalltalk",
            "Stella",
            "SugarCRM",
            "Swift",
            "Symfony",
            "SymphonyCMS",
            "TeX",
            "Terraform",
            "Textpattern",
            "TurboGears2",
            "TwinCAT3",
            "Typo3",
            "Unity",
            "UnrealEngine",
            "VVVV",
            "VisualStudio",
            "Waf",
            "WordPress",
            "Xojo",
            "Yeoman",
            "Yii",
            "ZendFramework",
            "Zephir",
        ];

        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&supported_technologies)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        if let Some(index) = selection {
            LogMessage::info(&format!("User selected: {}", supported_technologies[index]));
            let selection = supported_technologies[index];
            Self::generate_ignore_file_to_path(selection, full_path);
        } else {
            LogMessage::warning("No selection made. Operation cancelled.");
            std::process::exit(1);
        }

        Ok(())
    }

    fn generate_ignore_file_to_path(technology: &str, path: &Path) {
        let src_path = format!("gitignore/{}.gitignore", technology);
        if let Some(file) = TEMPLATES_DIR.get_file(src_path) {
            if let Ok(mut output) = File::create(path) {
                if let Err(err) = output.write_all(file.contents()) {
                    LogMessage::error(&format!("Failed to write .gitignore: {}", err));
                }
            } else {
                LogMessage::error("Unable to create .gitignore file.");
            }
        } else {
            LogMessage::warning(&format!(
                "No template found for technology '{}'.",
                technology
            ));
        }
    }
    fn create_readme_template(full_path: &Path) -> Result<(), FileSystemError> {
        let content = r#"
# Project Title

Simple overview of use/purpose.

## Description

An in-depth paragraph about your project and overview of use.

## Getting Started

### Dependencies

* Describe any prerequisites, libraries, OS version, etc., needed before installing program.
* ex. Windows 10

### Installing

* How/where to download your program
* Any modifications needed to be made to files/folders

### Executing program

* How to run the program
* Step-by-step bullets
```
code blocks for commands
```

## Help

Any advise for common problems or issues.
```
command to run if program contains helper info
```

## Authors

Contributors names and contact info

ex. Dominique Pizzie
ex. [@DomPizzie](https://twitter.com/dompizzie)

## Version History

* 0.2
    * Various bug fixes and optimizations
    * See [commit change]() or See [release history]()
* 0.1
    * Initial Release

## License

This project is licensed under the [NAME HERE] License - see the LICENSE.md file for details

## Acknowledgments

Inspiration, code snippets, etc.
* [awesome-readme](https://github.com/matiassingers/awesome-readme)
* [PurpleBooth](https://gist.github.com/PurpleBooth/109311bb0361f32d87a2)
* [dbader](https://github.com/dbader/readme-template)
* [zenorocha](https://gist.github.com/zenorocha/4526327)
* [fvcproductions](https://gist.github.com/fvcproductions/1bfc2d4aecb01a834b46)
        "#;
        let mut file = std::fs::File::create(full_path)
            .map_err(|err| FileSystemError::OperationError(err.to_string()))?;
        file.write_all(content.as_bytes())
            .map_err(|err| FileSystemError::OperationError(err.to_string()))?;
        Ok(())
    }
}
