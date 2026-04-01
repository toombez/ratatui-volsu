use ratatui::{DefaultTerminal, Frame};
use ratatui::widgets::StatefulWidgetRef;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{StatefulWidget, Widget};

struct PersonalGreeting;

impl StatefulWidgetRef for PersonalGreeting {
    type State = String;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Line::raw(format!("Hello {}", state)).render(area, buf);
    }
}

impl StatefulWidget for PersonalGreeting {
    type State = String;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        (&self).render_ref(area, buf, state);
    }
}

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        let widget = PersonalGreeting;

        terminal.draw(|frame| render(frame, widget))?;

        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, widget: PersonalGreeting) {
    frame.render_stateful_widget(widget, frame.area(), &mut String::from("World"));
}
