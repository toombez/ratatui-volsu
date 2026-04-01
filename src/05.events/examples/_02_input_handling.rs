use crossterm::ExecutableCommand;
use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{DefaultTerminal, Frame};
use ratatui::widgets::{Block, List, Paragraph};

#[derive(Default)]
struct App {
    input: String,
    messages: Vec<String>,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        std::io::stdout().execute(crossterm::event::EnableMouseCapture).unwrap();

        loop {
            terminal.draw(|frame| self.render(frame))?;

            let event = crossterm::event::read()?;

            match event {
                Event::Key(key_event) => {
                    if !key_event.is_press() {
                        continue
                    }

                    match key_event.code {
                        KeyCode::Esc => break Ok(()),
                        KeyCode::Backspace => {
                            if self.input.len() == 0 {
                                continue
                            }

                            let input: String = self.input.chars().take(self.input.len() - 1).collect();

                            self.input = input;
                        },
                        KeyCode::Enter => {
                            self.messages.push(self.input.clone());
                            self.input = String::new();
                        },
                        KeyCode::Char(char) => self.input.push(char),
                        _ => {}
                    }
                },
                Event::FocusLost => self.messages.push("Unfocused".to_string()),
                Event::FocusGained => self.messages.push("Focused".to_string()),
                Event::Mouse(event) => self.messages.push(format!("Mouse event: {event:?}")),
                Event::Resize(width, height) => self.messages.push(format!("New size: ({width}; {height})")),
                _ => {}
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let wrapper = Block::bordered().title("Input handling");

        let layout = Layout
            ::new(Direction::Vertical, vec![
                Constraint::Length(3),
                Constraint::Fill(1)
            ])
            .split(wrapper.inner(frame.area()));

        frame.render_widget(wrapper, frame.area());
        frame.render_widget(Paragraph
            ::new(self.input.clone())
            .block(Block::bordered().title("Input")),
            layout[0]
        );

        let history: Vec<String> = self.messages.clone().into_iter().rev().collect();

        frame.render_widget(List
            ::new(history)
            .block(Block::bordered().title("Message history")),
            layout[1],
        );
    }
}

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    App::default().run(terminal)?;
    Ok(())
}
