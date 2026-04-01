use crossterm::event::{KeyCode};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, List, ListState, Padding, Row, Table};

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;

        match crossterm::event::read()? {
            crossterm::event::Event::Key(key) => match key.code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => break Ok(()),
                _ => {},
            },
            _ => {},
        }
    }
}

fn render(frame: &mut Frame) {
    let layout = Layout
        ::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(frame.area());

    {
        let mut state = ListState::default();
        let items = ["Item 1", "Item 2", "Item 3"];
        let list = List::new(items)
            .block(Block::bordered().title("List"))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        state.select(Some(2));

        frame.render_stateful_widget(list, layout[0], &mut state);
    }

    {
        let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
        let widths = [
            Constraint::Length(30),
            Constraint::Length(5),
            Constraint::Length(10),
        ];
        let table = Table::new(rows, widths)
            .column_spacing(1)
            .header(Row::new(vec!["Col1", "Col2", "Col3"]))
            .footer(Row::new(vec!["Updated on Dec 28"]))
            .block(Block
                ::new()
                .title("Table")
                .padding(Padding::uniform(1))
                .borders(Borders::ALL)
            )
            .highlight_symbol(">>");

        frame.render_widget(table, layout[1]);
    }
}
