//! pypi-jump-to (pjt) - a quick navigation tool for the PyPI packages.

use console::style;
use pypi_jump_to::{commands, handlers};

fn main() {
    let cmd = handlers::args::JumpCommand::parse();
    if let Err(error) = commands::jump::execute(&cmd) {
        eprintln!("{} {}", style("Error:").red().bold(), error);
        std::process::exit(1);
    }
}
