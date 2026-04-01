use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

#[derive(Debug, Clone)]
pub enum Action {
    Increment,
    Decrement,
    IncrementBy(i32),
    Reset,
    UndoLast,
    Quit,
}

pub struct CounterStore {
    pub value: i32,
    pub history: Vec<String>,
    pub last_value: Option<i32>,
}

impl CounterStore {
    pub fn new() -> Self {
        Self { value: 0, history: Vec::new(), last_value: None }
    }

    pub fn reduce(&mut self, action: &Action) {
        let prev = self.value;

        match action {
            Action::Increment => {
                self.value = (self.value + 1).min(100);
                self.history.push(format!("+1  → {}", self.value));
            }
            Action::Decrement => {
                self.value = (self.value - 1).max(-100);
                self.history.push(format!("-1  → {}", self.value));
            }
            Action::IncrementBy(n) => {
                self.value = (self.value + n).clamp(-100, 100);
                self.history.push(format!("{:+}  → {}", n, self.value));
            }
            Action::Reset => {
                self.value = 0;
                self.history.push("RST → 0".to_string());
            }
            Action::UndoLast => {
                if let Some(prev_val) = self.last_value.take() {
                    self.value = prev_val;
                    self.history.push(format!("UNDO → {}", self.value));
                    return;
                }
            }
            Action::Quit => {}
        }

        if self.value != prev {
            self.last_value = Some(prev);
        }
    }
}

pub struct AppStore {
    pub running: bool,
    pub step: i32,
    pub action_count: u32,
}

impl AppStore {
    pub fn new() -> Self {
        Self { running: true, step: 1, action_count: 0 }
    }

    pub fn reduce(&mut self, action: &Action) {
        self.action_count += 1;
        match action {
            Action::Quit => self.running = false,
            Action::Increment | Action::Decrement | Action::IncrementBy(_) => {}
            _ => {}
        }
    }
}

pub struct Dispatcher {
    pub counter_store: CounterStore,
    pub app_store: AppStore,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            counter_store: CounterStore::new(),
            app_store: AppStore::new(),
        }
    }

    pub fn dispatch(&mut self, action: Action) {
        self.app_store.reduce(&action);
        self.counter_store.reduce(&action);
    }
}

fn view(dispatcher: &Dispatcher, frame: &mut Frame) {
    let counter = &dispatcher.counter_store;
    let app = &dispatcher.app_store;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(6),
            Constraint::Length(4),
        ])
        .split(frame.area());

    let color = match counter.value {
        v if v > 0 => Color::Green,
        v if v < 0 => Color::Red,
        _ => Color::Yellow,
    };

    let counter_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(1)])
        .split(
            Block::default()
                .title(" CounterStore ")
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Cyan))
                .inner(chunks[0]),
        );

    frame.render_widget(
        Block::default()
            .title(" CounterStore ")
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Cyan)),
        chunks[0],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("Value: "),
            Span::styled(
                format!("{:>5}", counter.value),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("   step: {}", app.step)),
            Span::raw(format!("   total actions: {}", app.action_count)),
        ])),
        counter_chunks[0],
    );

    let ratio = ((counter.value + 100) as f64 / 200.0).clamp(0.0, 1.0);
    frame.render_widget(
        Gauge::default()
            .gauge_style(Style::default().fg(color))
            .ratio(ratio)
            .label(format!("{}%", (ratio * 100.0) as u8)),
        counter_chunks[1],
    );

    let history_block = Block::default()
        .title(" Action History (CounterStore log) ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner_height = chunks[1].height.saturating_sub(2) as usize;
    let skip = counter.history.len().saturating_sub(inner_height);

    let items: Vec<ListItem> = counter
        .history
        .iter()
        .skip(skip)
        .enumerate()
        .map(|(i, entry)| {
            let idx = skip + i + 1;
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:>3}. ", idx), Style::default().fg(Color::DarkGray)),
                Span::styled(entry.clone(), Style::default().fg(Color::White)),
            ]))
        })
        .collect();

    frame.render_widget(List::new(items).block(history_block), chunks[1]);

    let help_lines = vec![
        Line::from(vec![
            Span::styled("↑/k", Style::default().fg(Color::Yellow)),
            Span::raw(" +1  "),
            Span::styled("↓/j", Style::default().fg(Color::Yellow)),
            Span::raw(" -1  "),
            Span::styled("+/-", Style::default().fg(Color::Yellow)),
            Span::raw(format!(" ±{}  ", app.step * 5)),
            Span::styled("u", Style::default().fg(Color::Yellow)),
            Span::raw(" Undo  "),
            Span::styled("r", Style::default().fg(Color::Yellow)),
            Span::raw(" Reset  "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(" Quit"),
        ]),
        Line::from(vec![
            Span::styled("AppStore: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                if app.running { "running" } else { "done" },
                Style::default().fg(if app.running { Color::Green } else { Color::Red }),
            ),
            Span::raw(format!("  dispatched: {} actions", app.action_count)),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(help_lines).block(
            Block::default()
                .title(" AppStore + Controls ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        ),
        chunks[2],
    );
}

pub fn example_app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut dispatcher = Dispatcher::new();

    while dispatcher.app_store.running {
        terminal.draw(|f| view(&dispatcher, f))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let step = dispatcher.app_store.step;
                    let action = match key.code {
                        KeyCode::Up | KeyCode::Char('k') => Some(Action::Increment),
                        KeyCode::Down | KeyCode::Char('j') => Some(Action::Decrement),
                        KeyCode::Char('+') => Some(Action::IncrementBy(step * 5)),
                        KeyCode::Char('-') => Some(Action::IncrementBy(-step * 5)),
                        KeyCode::Char('r') => Some(Action::Reset),
                        KeyCode::Char('u') => Some(Action::UndoLast),
                        KeyCode::Char('q') | KeyCode::Esc => Some(Action::Quit),
                        _ => None,
                    };

                    if let Some(a) = action {
                        dispatcher.dispatch(a);
                    }
                }
            }
        }
    }
    Ok(())
}
