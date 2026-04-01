use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    DefaultTerminal, Frame,
};

#[derive(Debug)]
pub enum AppEvent {
    Quit,
    FocusNext,
    FocusPrev,
}

pub trait Component {
    fn handle_key(&mut self, key: KeyEvent) -> Option<AppEvent>;
    fn render(&self, frame: &mut Frame, area: Rect, focused: bool);
    fn title(&self) -> &str;
}

pub struct CounterComponent {
    pub value: i32,
    pub step: i32,
    title: String,
}

impl CounterComponent {
    pub fn new(title: impl Into<String>, step: i32) -> Self {
        Self { value: 0, step, title: title.into() }
    }
}

impl Component for CounterComponent {
    fn handle_key(&mut self, key: KeyEvent) -> Option<AppEvent> {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.value = self.value + self.step;
                None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.value = self.value - self.step;
                None
            }
            KeyCode::Char('r') => {
                self.value = 0;
                None
            }
            KeyCode::Tab => Some(AppEvent::FocusNext),
            KeyCode::BackTab => Some(AppEvent::FocusPrev),
            KeyCode::Char('q') | KeyCode::Esc => Some(AppEvent::Quit),
            _ => None,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool) {
        let border_style = if focused {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .title(format!(" {} (step: {}) ", self.title, self.step))
            .borders(Borders::ALL)
            .border_type(if focused { BorderType::Double } else { BorderType::Rounded })
            .border_style(border_style);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Length(1), Constraint::Min(0)])
            .split(inner);

        let color = match self.value {
            v if v > 0 => Color::Green,
            v if v < 0 => Color::Red,
            _ => Color::Yellow,
        };
        let value_text = Paragraph::new(Line::from(vec![
            Span::raw("Value: "),
            Span::styled(
                format!("{:>5}", self.value),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
        ]));
        frame.render_widget(value_text, chunks[0]);
    }

    fn title(&self) -> &str {
        &self.title
    }
}

pub struct App {
    components: Vec<Box<dyn Component>>,
    focused: usize,
    quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            components: vec![
                Box::new(CounterComponent::new("Counter A", 1)),
                Box::new(CounterComponent::new("Counter B", 5)),
                Box::new(CounterComponent::new("Counter C", 100)),
            ],
            focused: 0,
            quit: false,
        }
    }

    fn handle_app_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Quit => self.quit = true,
            AppEvent::FocusNext => {
                self.focused = (self.focused + 1) % self.components.len();
            }
            AppEvent::FocusPrev => {
                self.focused =
                    (self.focused + self.components.len() - 1) % self.components.len();
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),
                Constraint::Length(7),
                Constraint::Length(7),
                Constraint::Length(5),
                Constraint::Min(0),
            ])
            .split(frame.area());

        for (i, component) in self.components.iter().enumerate() {
            component.render(frame, chunks[i], i == self.focused);
        }
    }
}

pub fn example_app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();

    while !app.quit {
        terminal.draw(|f| app.draw(f))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let app_event = app.components[app.focused].handle_key(key);
                    if let Some(ev) = app_event {
                        app.handle_app_event(ev);
                    }
                }
            }
        }
    }
    Ok(())
}
