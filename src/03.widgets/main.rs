pub mod cli;
use cli::{Cli, Parser, WidgetExample};

pub mod examples;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let app = match cli.widget {
        WidgetExample::Text => examples::_01_text::example_app,
        WidgetExample::List => examples::_02_list::example_app,
        WidgetExample::Data => examples::_03_data::example_app,
    };

    ratatui::run(app)?;

    Ok(())
}
