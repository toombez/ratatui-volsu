use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Paragraph};

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
            Constraint::Length(8),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .split(frame.area());

    frame.render_widget(
        Paragraph
            ::new(vec![
                Line::from("Red text".red()),
                Line::from("Red background".on_red()),
                Line::from("Combined".on_cyan().magenta()),
                Line::from("RGB color"
                    .bg(Color::Rgb(128, 191, 134))
                    .fg(Color::Rgb(0, 0, 0))
                ),
                Line::from("Reversed color".reversed()),
                Line::from("Dim color".dim()),
            ])
            .block(Block::bordered().title("Color styles")),
        layout[0]
    );

    frame.render_widget(
        Line
            ::from("Full length line style")
            .style(Style::new().bg(Color::LightGreen).fg(Color::Red)),
        layout[1]
    );

    frame.render_widget(
        Block
            ::bordered()
            .title("Border styles")
            .border_style(Style::new().fg(Color::Magenta)),
        layout[2]
    );
}
