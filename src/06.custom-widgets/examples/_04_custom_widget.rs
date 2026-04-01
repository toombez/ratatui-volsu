use ratatui::buffer::Buffer;
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use ratatui::text::Line;

struct MyWidget;

impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::raw("Hello").render(area, buf);
    }
}

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        let widget = MyWidget;

        terminal.draw(|frame| render(frame, widget))?;

        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, widget: MyWidget) {
    frame.render_widget(widget, frame.area());
}
