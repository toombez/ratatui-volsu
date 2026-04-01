use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
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
        let layout = Layout
            ::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(20),
            ])
            .split(Rect::new(0, 0, frame.area().width, 3))
        ;

        frame.render_widget(
            Paragraph
                ::new("50%")
                .block(Block::new().borders(Borders::ALL)),
            layout[0]
        );
        frame.render_widget(
            Paragraph
                ::new("20%")
                .block(Block::new().borders(Borders::ALL)),
            layout[1]
        );
    };

    {
        let layout = Layout
            ::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(10),
                Constraint::Length(40),
            ])
            .split(Rect::new(0, 3, frame.area().width, 3))
        ;

        frame.render_widget(
            Paragraph
                ::new("10 units")
                .block(Block::new().borders(Borders::ALL)),
            layout[0]
        );
        frame.render_widget(
            Paragraph
                ::new("40 units")
                .block(Block::new().borders(Borders::ALL)),
            layout[1]
        );
    };

    {
        let layout = Layout
            ::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Fill(2),
            ])
            .split(Rect::new(0, 6, frame.area().width, 3))
        ;

        frame.render_widget(
            Paragraph
                ::new("fill 1")
                .block(Block::new().borders(Borders::ALL)),
            layout[0]
        );
        frame.render_widget(
            Paragraph
                ::new("fill 2")
                .block(Block::new().borders(Borders::ALL)),
            layout[1]
        );
    };

    {
        let layout = Layout
            ::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(10),
                Constraint::Percentage(20),
                Constraint::Fill(1),
                Constraint::Fill(2),
            ])
            .split(Rect::new(0, 9, frame.area().width, 3))
        ;

        frame.render_widget(
            Paragraph
                ::new("10 units")
                .block(Block::new().borders(Borders::ALL)),
            layout[0]
        );
        frame.render_widget(
            Paragraph
                ::new("20%")
                .block(Block::new().borders(Borders::ALL)),
            layout[1]
        );
        frame.render_widget(
            Paragraph
                ::new("fill 1")
                .block(Block::new().borders(Borders::ALL)),
            layout[2]
        );
        frame.render_widget(
            Paragraph
                ::new("fill 2")
                .block(Block::new().borders(Borders::ALL)),
            layout[3]
        );
    }
}
