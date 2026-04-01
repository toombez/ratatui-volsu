use ratatui::{DefaultTerminal, Frame, layout::{Alignment, Constraint, Direction, Layout}, style::Color, widgets::{Block, Paragraph, Wrap}};
use tachyonfx::{Duration, Effect, Motion, fx};

use std::{
    time::{Duration as StdDuration},
};

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut parallel_effect = fx::parallel(&[
        fx::fade_from_fg(Color::Red, 500),
        fx::sweep_in(Motion::LeftToRight, 10, 0, Color::Black, 800),
    ]);

    let mut sequence_effect = fx::sequence(&[
        fx::fade_from_fg(Color::Black, 300),
        fx::coalesce(500),
    ]);

    loop {
        terminal.draw(|frame| render(frame, &mut parallel_effect, &mut sequence_effect))?;

        if crossterm::event::poll(StdDuration::from_millis(100))? {
            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            }
        }
    }
}

fn render(frame: &mut Frame, parallel_effect: &mut Effect, sequence_effect: &mut Effect) {
    let area = frame.area();

    frame.render_widget(Block::bordered().title("Combed"), frame.area());

    let layout = Layout::new(Direction::Vertical, vec![Constraint::Fill(1); 2]).split(area);

    let text = String::from("Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatibus perspiciatis laudantium molestiae, ex eos harum, quasi magnam aliquid dolorem molestias at numquam magni eveniet! Nostrum officiis quas fugit expedita laudantium eaque molestias iusto, placeat, numquam quidem, ratione natus! Qui modi sint asperiores, recusandae, labore dicta aut quod, quam eius quisquam ad repellendus facere? Odio laborum quos sed omnis quae. Possimus quos fugiat fugit, debitis qui dolore maiores corporis autem perferendis sint excepturi quae molestias sunt officia aut ab asperiores reiciendis illo labore adipisci. Perspiciatis similique doloribus numquam consectetur ullam quos pariatur, a deleniti atque eligendi optio aspernatur dolorem dolore sunt dicta! Facilis rerum distinctio, minima debitis expedita magni obcaecati corrupti possimus maiores. Cumque consectetur esse culpa, atque iusto voluptates placeat? Sunt commodi rem, dignissimos repudiandae quisquam expedita, quam ad provident doloribus maiores libero, nulla fuga eaque ratione. Repudiandae, eveniet accusamus neque distinctio quia soluta quasi veniam maiores quis eum aperiam dolorem voluptatum optio! Quasi minus vitae excepturi rerum cupiditate reiciendis ipsam porro incidunt dignissimos provident officia adipisci deserunt rem a, dicta, eius qui exercitationem quam minima maxime veniam dolorem dolores. Magnam vitae non officia cum eveniet enim alias quidem aliquam praesentium voluptatem, impedit quasi temporibus, quaerat architecto at mollitia tempore.");

    frame.render_widget(Paragraph::new(text.clone()).wrap(Wrap{ trim: true }).alignment(Alignment::Center), layout[0]);
    frame.render_widget(Paragraph::new(text).wrap(Wrap{ trim: true }).alignment(Alignment::Center), layout[1]);

    if parallel_effect.running() {
        parallel_effect.process(Duration::from_millis(33), frame.buffer_mut(), layout[0]);
    }

    if sequence_effect.running() {
        sequence_effect.process(Duration::from_millis(33), frame.buffer_mut(), layout[1]);
    }
}
