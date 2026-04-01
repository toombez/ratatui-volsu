use std::{
    io,
    time::{Duration as StdDuration, Instant},
};

use ratatui::{
    DefaultTerminal, crossterm::event, prelude::*, widgets::{Block, Clear}
};
use tachyonfx::{
    CenteredShrink, Duration, Effect, EffectRenderer, EffectTimer, Interpolation, IntoEffect,
    Motion, SimpleRng,
    fx,
};

pub fn example_app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let running_time = Instant::now();

    let mut glitch: Effect = fx::Glitch::builder()
        .rng(SimpleRng::default())
        .action_ms(200..400)
        .action_start_delay_ms(0..1)
        .cell_glitch_ratio(1.0)
        .build()
        .into_effect();

    let mut effect = fx::sequence(&[
        fx::ping_pong(fx::sweep_in(
            Motion::LeftToRight,
            10,
            0,
            Color::LightCyan,
            EffectTimer::from_ms(2000, Interpolation::QuadIn),
        )),
        fx::coalesce((800, Interpolation::SineOut)),
    ]);

    loop {
        let effect = if running_time.elapsed() > StdDuration::from_secs(10) {
            &mut glitch
        } else {
            &mut effect
        };

        terminal.draw(|f| render(f, effect))?;

        if event::poll(StdDuration::from_millis(100))? {
            if crossterm::event::read()?.is_key_press() {
                break;
            }
        }
    }

    Ok(())
}

fn render(frame: &mut Frame<'_>, effect: &mut Effect) {
    Clear.render(frame.area(), frame.buffer_mut());

    Block::default()
        .style(Style::default().bg(Color::DarkGray))
        .render(frame.area(), frame.buffer_mut());

    let area = frame.area().inner_centered(25, 2);
    let main_text = Text::from(vec![
        Line::from("Hello, TachyonFX!"),
        Line::from("Press any key to exit."),
    ]);

    frame.render_widget(main_text.white().centered(), area);

    if effect.running() {
        frame.render_effect(effect, area, Duration::from_millis(33));
    }
}
