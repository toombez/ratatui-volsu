use ratatui::symbols::merge::MergeStrategy;
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Direction, Layout, Spacing};
use ratatui::widgets::{Block, BorderType, Borders};

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;

        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let layout = Layout
        ::new(Direction::Vertical, vec![
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(6),
            Constraint::Length(2),
        ])
        .spacing(Spacing::Space(1))
        .split(frame.area());

    let combined_horizontal_borders_layout = Layout
        ::new(Direction::Horizontal, vec![Constraint::Fill(1); 2])
        .spacing(Spacing::Overlap(1))
        .split(layout[2]);

    let combined_vertical_borders_layout = Layout
        ::new(Direction::Vertical, vec![Constraint::Fill(1); 2])
        .spacing(Spacing::Overlap(1))
        .split(combined_horizontal_borders_layout[1]);

    frame.render_widget(
        Block::bordered().title("Borders all"),
        layout[0],
    );

    frame.render_widget(
        Block::default()
        .borders(Borders::LEFT.union(Borders::RIGHT))
        .title("Intersect borders"),
        layout[1],
    );

    frame.render_widget(
        Block::bordered().border_type(BorderType::HeavyDoubleDashed).title("Border type"),
        layout[2],
    );

    frame.render_widget(
        Block::bordered().title("Collapsed borders").merge_borders(MergeStrategy::Exact),
        combined_horizontal_borders_layout[0],
    );
    frame.render_widget(
        Block::bordered().merge_borders(MergeStrategy::Exact),
        combined_vertical_borders_layout[0],
    );
    frame.render_widget(
        Block::bordered().merge_borders(MergeStrategy::Exact),
        combined_vertical_borders_layout[1],
    );
    frame.render_widget(
        Block::bordered().border_type(BorderType::Double),
        layout[3],
    );
}
