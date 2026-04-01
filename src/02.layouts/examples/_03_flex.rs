use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
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
    layout(Flex::Start, frame, 0);
    layout(Flex::Center, frame, 3);
    layout(Flex::End, frame, 6);
    layout(Flex::SpaceAround, frame, 9);
    layout(Flex::SpaceBetween, frame, 12);
    layout(Flex::SpaceEvenly, frame, 15);
}

fn layout(flex: Flex, frame: &mut Frame, y: u16) {
    let layout = Layout
        ::default()
        .direction(Direction::Horizontal)
        .flex(flex)
        .constraints(vec![
            Constraint::Length(10),
            Constraint::Length(20),
            Constraint::Length(30),
        ])
        .split(Rect::new(0, y, frame.area().width, 3))
    ;

    frame.render_widget(
        Paragraph
            ::new("10 units")
            .block(Block::new().borders(Borders::ALL)),
        layout[0],
    );

    frame.render_widget(
        Paragraph
            ::new("20 units")
            .block(Block::new().borders(Borders::ALL)),
        layout[1],
    );

    frame.render_widget(
        Paragraph
            ::new("30 units")
            .block(Block::new().borders(Borders::ALL)),
        layout[2],
    );
}
