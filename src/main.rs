//! pypi-jump-to (pjt) - a quick navigation tool for the PyPI packages.

use console::style;

pub mod commands;
pub mod handlers;

fn main() {
    let cmd = handlers::args::JumpCommand::parse();
    if let Err(e) = commands::jump::execute(&cmd) {
        eprintln!("{} {}", style("Error:").red().bold(), e);
        std::process::exit(1);
    }
}
