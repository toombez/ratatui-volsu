use crossterm::event::{Event, KeyCode};
use ratatui::{DefaultTerminal, Frame};
use ratatui::widgets::{Block, Paragraph};

#[derive(Debug, Default)]
struct AppState {
    last_char: Option<char>
}

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut state = AppState::default();

    loop {
        terminal.draw(|frame| render(frame, state.last_char))?;

        let event: Event = crossterm::event::read()?;

        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Esc => break Ok(()),
                KeyCode::Char(char) => state.last_char = Some(char),
                _ => {}
            },
            _ => {}
        }
    }
}

fn render(frame: &mut Frame, char: Option<char>) {
    let message = format!("Last char: {char:?}");

    frame.render_widget(
        Paragraph
            ::new(message)
            .centered()
            .block(Block::bordered()),
        frame.area(),
    );
}
