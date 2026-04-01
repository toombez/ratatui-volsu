use ratatui::style::{Color, Style};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Rect};
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;

        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    {
        let paragraph = Paragraph
            ::new("Positional block 1")
            .block(Block
                ::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(Color::Cyan))
            );

        let position = (0, 0);
        let size = (30, 10);

        frame.render_widget(
            paragraph,
            Rect::new(position.0, position.1, size.0, size.1)
        );
    };

    {
        let paragraph = Paragraph
            ::new("Positional block 2")
            .block(Block
                ::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(Color::LightMagenta))
            );

        let position = (10, 4);
        let size = (40, 5);

        frame.render_widget(
            paragraph,
            Rect::new(position.0, position.1, size.0, size.1)
        );
    };

    {
        let paragraph = Paragraph
            ::new("Positional block 3")
            .block(Block
                ::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(Color::Green))
            );

        let position = (0, 20);
        let size = (frame.area().width, 5);

        frame.render_widget(
            paragraph,
            Rect::new(position.0, position.1, size.0, size.1)
        );
    };

    {
        let paragraph = Paragraph
            ::new("Positional block 4")
            .block(Block
                ::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(Color::LightRed))
            );

        let position = (70, 0);
        let size = (25, frame.area().height);

        frame.render_widget(
            paragraph,
            Rect::new(position.0, position.1, size.0, size.1)
        );
    }
}
