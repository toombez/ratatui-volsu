use ratatui::text::{Line, Span};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;

        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget(
        Block
            ::new()
            .title("Block title")
            .title_bottom("Bottom block title")
            .borders(Borders::ALL),
        frame.area()
    );

    frame.render_widget(
        Paragraph
            ::new("Inner paragraph with padding 2")
            .block(Block
                ::new()
                .title("Paragraph block title")
                .borders(Borders::ALL)
                .padding(Padding::uniform(2)),
            ),
        Rect::new(4, 5, frame.area().width / 2, 10)
    );

    frame.render_widget(
        Paragraph
            ::new(vec![
                Line::from(vec![
                    Span::raw("First"), " ".into(),
                    Span::raw("line"),
                    ".".into(),
                ]),
                Line::from("Second line"),
                "Third line".into(),
            ])
            .block(Block::bordered().title("Paragraph"))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        Rect::new(8, 15, frame.area().width / 2, 10)
    );
}
