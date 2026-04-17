use std::{cell::RefCell, io, rc::Rc};

use ratzilla::ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Terminal,
};

use ratzilla::{event::KeyCode, DomBackend, WebRenderer};

fn main() -> io::Result<()> {
    let counter = Rc::new(RefCell::new(0i32));
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    terminal.on_key_event({
        let counter = counter.clone();
        move |key_event| match key_event.code {
            KeyCode::Char(' ') | KeyCode::Up => {
                let mut c = counter.borrow_mut();
                *c += 1;
            }
            KeyCode::Down => {
                let mut c = counter.borrow_mut();
                *c -= 1;
            }
            KeyCode::Char('r') => {
                let mut c = counter.borrow_mut();
                *c = 0;
            }
            _ => {}
        }
    });

    terminal.draw_web(move |f| {
        let counter = counter.borrow();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(7),
                Constraint::Fill(1),
            ])
            .split(f.area());

        let center = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(40),
                Constraint::Fill(1),
            ])
            .split(chunks[1])[1];

        let color = match *counter {
            v if v > 0 => Color::Green,
            v if v < 0 => Color::Red,
            _ => Color::Yellow,
        };

        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("  Count: "),
                Span::styled(
                    format!("{:>5}", *counter),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(
                Span::styled("  Space/↑ +1 | ↓ -1 | R reset", Style::default().fg(Color::DarkGray)),
            ),
        ];

        f.render_widget(
            Paragraph::new(text)
                .alignment(Alignment::Left)
                .block(
                    Block::bordered()
                        .title(" Ratatui in the Browser ")
                        .title_alignment(Alignment::Center)
                        .border_style(Color::Cyan),
                ),
            center,
        );
    });

    Ok(())
}
