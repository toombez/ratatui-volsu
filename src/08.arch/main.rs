pub mod examples;
pub mod cli;
use cli::{Cli, Parser, ArchExample};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let app = match cli.arch {
        ArchExample::Elm => examples::_01_elm::example_app,
        ArchExample::Component => examples::_02_component::example_app,
        ArchExample::Flux => examples::_03_flux::example_app,
    };

    ratatui::run(app)?;

    Ok(())
}
