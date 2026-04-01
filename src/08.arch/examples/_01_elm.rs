use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame, widgets::{Block, Paragraph}};

#[derive(PartialEq)]
pub enum Message {
    Increment,
    Decrement,
    Reset,
    Quit,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default)]
pub struct Model {
    pub counter: i32,
    pub running_state: RunningState,
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Increment => {
            model.counter += 1;
            if model.counter > 50 {
                return Some(Message::Reset);
            }
        }
        Message::Decrement => {
            model.counter -= 1;
            if model.counter < -50 {
                return Some(Message::Reset);
            }
        }
        Message::Reset => model.counter = 0,
        Message::Quit => {
            model.running_state = RunningState::Done;
        }
    };
    None
}

fn view(model: &mut Model, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!("Counter: {}", model.counter)).block(Block::bordered().title("Counter elm arch example")),
        frame.area(),
    );
}

fn handle_event(_: &Model) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('j') | KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Tab => Some(Message::Increment),
        KeyCode::Char('k') | KeyCode::Backspace | KeyCode::BackTab => Some(Message::Decrement),
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => Some(Message::Quit),
        _ => None,
    }
}

pub fn example_app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut model = Model::default();

    while model.running_state != RunningState::Done {
        terminal.draw(|f| view(&mut model, f))?;

        let mut current_msg = handle_event(&model).unwrap();

        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    Ok(())
}
