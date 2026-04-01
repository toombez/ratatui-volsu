pub mod cli;
use cli::{Cli, Parser, StyleExample};

pub mod examples;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let app = match cli.style {
        StyleExample::Text => examples::_01_text::example_app,
        StyleExample::Borders => examples::_02_borders::example_app,
        StyleExample::Colors => examples::_03_colors::example_app,
    };

    ratatui::run(app)?;

    Ok(())
}
