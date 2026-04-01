pub mod cli;
use cli::{Cli, Parser, EventExample};

pub mod examples;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let app = match cli.event {
        EventExample::BackendRead => examples::_01_backend_read::example_app,
        EventExample::InputHandling => examples::_02_input_handling::example_app,
    };

    ratatui::run(app)?;

    Ok(())
}
