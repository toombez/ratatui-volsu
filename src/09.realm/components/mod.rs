use tuirealm::props::{Alignment, Borders, Color, Style};
use tuirealm::ratatui::widgets::Block;

use super::Msg;

mod clock;
mod counter;
mod label;

pub use clock::Clock;
pub use counter::{DigitCounter, LetterCounter};
pub use label::Label;

pub(crate) fn get_block(props: Borders, title: (String, Alignment), focus: bool) -> Block<'static> {
    Block::default()
        .borders(props.sides)
        .border_style(if focus {
            props.style()
        } else {
            Style::default().fg(Color::Reset).bg(Color::Reset)
        })
        .border_type(props.modifiers)
        .title(title.0)
        .title_alignment(title.1)
}
