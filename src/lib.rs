#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
pub mod commands;
pub mod config;
pub mod constants;
/// The line `// pub mod database;` is a commented-out line of code in Rust. This means that it is not
/// active or being used in the current codebase. It appears that the `database` module is not being
/// included or imported in the current project.
// pub mod database;
pub mod errors;
pub mod helpers;
pub mod parsers;
pub mod workers;

pub mod forge;
