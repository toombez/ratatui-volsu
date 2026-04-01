pub mod cli;
use cli::{Cli, Parser, LayoutExample};

pub mod examples;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let app = match cli.layout {
        LayoutExample::Positional => examples::_01_positional::example_app,
        LayoutExample::Constraint => examples::_02_constraints::example_app,
        LayoutExample::Flex => examples::_03_flex::example_app,
        LayoutExample::Nested => examples::_04_nested::example_app,
    };

    ratatui::run(app)?;

    Ok(())
}
