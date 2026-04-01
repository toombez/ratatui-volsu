use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Direction, Flex, Layout, Spacing};
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
    let root_layout = Layout
        ::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(5),
            Constraint::Percentage(10),
            Constraint::Fill(1),
        ])
        .split(frame.area())
    ;

    let nested_layout_top = Layout
        ::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Length(20),
        ])
        .spacing(Spacing::Space(5))
        .split(root_layout[0])
    ;

    let nested_layout_bottom = Layout
        ::default()
        .direction(Direction::Horizontal)
        .flex(Flex::SpaceBetween)
        // .spacing(Spacing::Space(2))
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Length(20),
        ])
        .split(root_layout[2])
    ;

    frame.render_widget(
        Paragraph
            ::new("50%")
            .block(Block::new().borders(Borders::ALL)),
        nested_layout_top[0],
    );
    frame.render_widget(
        Paragraph
            ::new("20%")
            .block(Block::new().borders(Borders::ALL)),
        nested_layout_top[1],
    );

    frame.render_widget(
        Paragraph
            ::new("center")
            .block(Block::new().borders(Borders::ALL)),
        root_layout[1],
    );

    frame.render_widget(
        Paragraph
            ::new("50%")
            .block(Block::new().borders(Borders::ALL)),
        nested_layout_bottom[0],
    );
    frame.render_widget(
        Paragraph
            ::new("20%")
            .block(Block::new().borders(Borders::ALL)),
        nested_layout_bottom[1],
    );
}
