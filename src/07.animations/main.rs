pub mod cli;
use cli::{Cli, Parser, AnimationExample};

pub mod examples;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let app = match cli.animation {
        AnimationExample::Minimal => examples::_01_minimal::example_app,
        AnimationExample::Paint => examples::_02_paint::example_app,
        AnimationExample::Combined => examples::_03_combed::example_app,
    };

    ratatui::run(app)?;

    Ok(())
}
