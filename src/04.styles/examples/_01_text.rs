use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Alignment, Constraint, Direction, Flex, Layout};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;

        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let block = Block
        ::new()
        .borders(Borders::ALL)
        .title("Text styles");

    frame.render_widget(
        block.clone(),
        frame.area()
    );

    let layout = Layout
        ::default()
        .direction(Direction::Vertical)
        .flex(Flex::Start)
        .constraints(vec![
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Fill(1),
        ])
        .split(block.inner(frame.area()));

    frame.render_widget(
        Paragraph
            ::new(Line::from(vec![
                Span::from("Normal"), Span::from(" "),
                Span::from("Italic".italic()), Span::from(" "),
                Span::from("Bold".bold())
            ]))
            .block(Block
                ::new()
                .padding(Padding::top(1))
                .borders(Borders::all())
                .title("Font styles")
            ),
        layout[0]
    );

    frame.render_widget(
        Paragraph
            ::new(Line::from(vec![
                Span::from("Underlined".underlined()), Span::from(" "),
                Span::from("Crossed".crossed_out()),
            ]))
            .block(Block
                ::new()
                .padding(Padding::top(1))
                .borders(Borders::all())
                .title("Text decoration")
            ),
        layout[1]
    );

    frame.render_widget(
        Paragraph
            ::new(Line::from(vec![
                Span::from("Underlined bold".underlined().bold()), Span::from(" "),
                Span::from("Crossed italic".crossed_out().italic()),
            ]))
            .block(Block
                ::new()
                .padding(Padding::top(1))
                .borders(Borders::all())
                .title("Mixed")
            ),
        layout[2]
    );

    frame.render_widget(
        Paragraph
            ::new(vec![
                Line::from("Left alignment").alignment(Alignment::Left),
                Line::from("Center alignment").alignment(Alignment::Center),
                Line::from("Right alignment").alignment(Alignment::Right),
            ])
            .block(Block
                ::new()
                .padding(Padding::top(1))
                .borders(Borders::all())
                .title("Text alignment")
            ),

        layout[3]
    );
}
