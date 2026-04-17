use std::time::{Duration, SystemTime};

use tuirealm::event::NoUserEvent;
use tuirealm::props::{Alignment, Color, TextModifiers};
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge};
use tuirealm::{
    Application, AttrValue, Attribute, EventListenerCfg, Sub, SubClause, SubEventClause, Update,
};

use super::components::{Clock, DigitCounter, Label, LetterCounter};
use super::{Id, Msg};

pub struct Model<T>
where
    T: TerminalAdapter,
{
    pub app: Application<Id, Msg, NoUserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge<T>,
}

impl Default for Model<CrosstermTerminalAdapter> {
    fn default() -> Self {
        Self {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::init_crossterm().expect("Cannot initialize terminal"),
        }
    }
}

impl<T> Model<T>
where
    T: TerminalAdapter,
{
    pub fn view(&mut self) {
        let _ = self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .split(f.area());

            self.app.view(&Id::Clock, f, chunks[0]);
            self.app.view(&Id::LetterCounter, f, chunks[1]);
            self.app.view(&Id::DigitCounter, f, chunks[2]);
            self.app.view(&Id::Label, f, chunks[3]);
        });
    }

    fn init_app() -> Application<Id, Msg, NoUserEvent> {
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(Duration::from_millis(20), 3)
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );

        // Монтируем компоненты
        let _ = app.mount(
            Id::Label,
            Box::new(
                Label::default()
                    .text("Press letters or digits. Tab to switch focus. Esc to quit.")
                    .alignment(Alignment::Left)
                    .background(Color::Reset)
                    .foreground(Color::LightYellow)
                    .modifiers(TextModifiers::BOLD),
            ),
            Vec::default(),
        );

        let _ = app.mount(
            Id::Clock,
            Box::new(
                Clock::new(SystemTime::now())
                    .alignment(Alignment::Center)
                    .background(Color::Reset)
                    .foreground(Color::Cyan)
                    .modifiers(TextModifiers::BOLD),
            ),
            vec![Sub::new(SubEventClause::Tick, SubClause::Always)],
        );

        let _ = app.mount(
            Id::LetterCounter,
            Box::new(LetterCounter::new(0)),
            Vec::new(),
        );

        let _ = app.mount(
            Id::DigitCounter,
            Box::new(DigitCounter::new(0)),
            Vec::default(),
        );

        let _ = app.active(&Id::LetterCounter);
        app
    }
}

impl<T> Update<Msg> for Model<T>
where
    T: TerminalAdapter,
{
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            self.redraw = true;
            match msg {
                Msg::AppClose => {
                    self.quit = true;
                    None
                }
                Msg::ClockTick => None,
                Msg::DigitCounterBlur => {
                    let _ = self.app.active(&Id::LetterCounter);
                    None
                }
                Msg::DigitCounterChanged(v) => {
                    let _ = self.app.attr(
                        &Id::Label,
                        Attribute::Text,
                        AttrValue::String(format!("Digit counter: {v}")),
                    );
                    None
                }
                Msg::LetterCounterBlur => {
                    let _ = self.app.active(&Id::DigitCounter);
                    None
                }
                Msg::LetterCounterChanged(v) => {
                    let _ = self.app.attr(
                        &Id::Label,
                        Attribute::Text,
                        AttrValue::String(format!("Letter counter: {v}")),
                    );
                    None
                }
            }
        } else {
            None
        }
    }
}
