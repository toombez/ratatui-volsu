use ratatui::{DefaultTerminal, Frame, layout::Rect, style::Color, widgets::Block};
use tachyonfx::{Duration, Effect, fx, pattern::DiagonalPattern};

use std::{
    time::{Duration as StdDuration},
};

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let screen_size = terminal.size()?;

    let content_area = Rect::new(0, 0, screen_size.width, screen_size.height);
    let fg = Color::from_u32(0xfe8019);
    let bg = Color::from_u32(0x1d2021);

    let mut effect = fx::paint(fg, bg, 1000)
        .with_area(content_area)
        .with_pattern(DiagonalPattern::bottom_left_to_top_right());

    loop {
        terminal.draw(|frame| render(frame, &mut effect))?;

        if crossterm::event::poll(StdDuration::from_millis(100))? {
            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            }
        }
    }
}

fn render(frame: &mut Frame, effect: &mut Effect) {
    frame.render_widget(Block::bordered().title("Paint effect"), frame.area());

    let area = frame.area();

    if effect.running() {
        effect.process(Duration::from_millis(33), frame.buffer_mut(), area);
    }
}
