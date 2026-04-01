pub mod cli;
use cli::{Cli, Parser, CustomWidgetExample};

pub mod examples;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let app = match cli.widget {
        CustomWidgetExample::Explorer => examples::_01_third_party_explorer::example_app,
        CustomWidgetExample::QrCode => examples::_02_third_party_qr::example_app,
        CustomWidgetExample::PieChart => examples::_03_third_party_piechart::example_app,
        CustomWidgetExample::CustomWidget => examples::_04_custom_widget::example_app,
        CustomWidgetExample::StatefullWidget => examples::_05_statefull_widget::example_app,
    };

    ratatui::run(app)?;

    Ok(())
}
