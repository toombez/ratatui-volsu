use crossterm::event::{Event, KeyCode};
use ratatui::{DefaultTerminal, Frame, style::Color};
use tui_piechart::{PieChart, PieSlice};

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let slices = vec![
        PieSlice::new("Rust", 45.0, Color::Red),
        PieSlice::new("Go", 30.0, Color::Blue),
        PieSlice::new("Python", 25.0, Color::Green),
    ];

    let piechart = PieChart::new(slices);

    loop {
        terminal.draw(|frame| render(frame, &piechart))?;

        let event = crossterm::event::read()?;

        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break Ok(()),
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, piechart: &PieChart) {
    frame.render_widget(piechart, frame.area());
}
