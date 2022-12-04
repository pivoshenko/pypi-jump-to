//! Command-line argument definitions and parsing.

use clap::{Parser, ValueEnum};
use console;

#[derive(ValueEnum, Clone, Debug)]
pub enum Destination {
    #[value(alias = "c")]
    #[value(help = format!("(alias: {}) Changelog", console::style("c").cyan().bold()))]
    Changelog,

    #[value(alias = "d")]
    #[value(help = format!("(alias: {}) Documentation", console::style("d").cyan().bold()))]
    Documentation,

    #[value(alias = "g")]
    #[value(help = format!("(alias: {}) GitHub repository", console::style("g").cyan().bold()))]
    Github,

    #[value(alias = "h")]
    #[value(help = format!("(alias: {}) Homepage", console::style("h").cyan().bold()))]
    Homepage,

    #[value(alias = "i")]
    #[value(help = format!("(alias: {}) GitHub issues", console::style("i").cyan().bold()))]
    Issues,

    #[value(alias = "p")]
    #[value(help = format!("(alias: {}) GitHub pull requests", console::style("p").cyan().bold()))]
    PullRequests,

    #[value(alias = "r")]
    #[value(help = format!("(alias: {}) GitHub releases", console::style("r").cyan().bold()))]
    Releases,

    #[value(alias = "t")]
    #[value(help = format!("(alias: {}) GitHub tags", console::style("t").cyan().bold()))]
    Tags,

    #[value(alias = "v")]
    #[value(help = format!("(alias: {}) PyPI versions", console::style("v").cyan().bold()))]
    Versions,
}

fn build_examples_section() -> String {
    format!(
        "\n{}\n    {} {}\n    {} → {}\n    {} {}\n    {} → {}\n    {} {}\n    {} → {}",
        console::style("Examples:").green().bold(),
        console::style("pjt httpx").cyan(),
        console::style("(no specified destination)").dim(),
        console::style("🐙").green(),
        console::style("https://pypi.org/project/httpx")
            .blue()
            .underlined(),
        console::style("pjt django g").cyan(),
        console::style("(GitHub repository)").dim(),
        console::style("🐙").green(),
        console::style("https://github.com/django/django")
            .blue()
            .underlined(),
        console::style("pjt requests d").cyan(),
        console::style("(Documentation)").dim(),
        console::style("🐙").green(),
        console::style("https://requests.readthedocs.io/en/latest")
            .blue()
            .underlined()
    )
}

#[derive(Parser)]
#[command(name = "pjt")]
#[command(version = "1.0.0")]
#[command(author = "Volodymyr Pivoshenko <volodymyr.pivoshenko@gmail.com>")]
#[command(about = "pypi-jump-to (pjt) - a quick navigation tool for the PyPI packages")]
#[command(styles = clap::builder::Styles::styled()
    .header(clap::builder::styling::AnsiColor::Green.on_default().bold())
    .usage(clap::builder::styling::AnsiColor::Green.on_default().bold())
    .literal(clap::builder::styling::AnsiColor::Cyan.on_default())
    .placeholder(clap::builder::styling::AnsiColor::Magenta.on_default())
)]
#[command(after_help = build_examples_section())]
pub struct JumpCommand {
    #[arg(help = "Name of the package (e.g., httpx, django, numpy)")]
    pub package: String,

    #[arg(value_enum, default_value_t = Destination::Homepage, help = "Destination to jump to")]
    pub destination: Destination,
}

impl JumpCommand {
    pub fn parse() -> Self {
        Parser::parse()
    }
}
