use crossterm::event::{Event, KeyCode};
use ratatui::style::{Color, Modifier, Style};
use ratatui::{DefaultTerminal, Frame};
use ratatui::widgets::{Block, BorderType, Borders, FrameExt};
use ratatui_explorer::{FileExplorer, FileExplorerBuilder, Theme};

// https://github.com/ratatui/awesome-ratatui

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut file_explorer = FileExplorerBuilder::build_with_theme(Theme
        ::default()
        .with_title_bottom(|fe| format!("[{} files]", fe.files().len()).into())
        .with_block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
        .with_highlight_item_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .with_highlight_dir_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .with_highlight_symbol("> ".into())
    )?;

    loop {
        terminal.draw(|frame| render(frame, &mut file_explorer))?;

        let event = crossterm::event::read()?;

        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break Ok(()),
                _ => {}
            }
        }

        file_explorer.handle(&event)?;
    }
}

fn render(frame: &mut Frame, file_explorer: &mut FileExplorer) {
    frame.render_widget_ref(file_explorer.widget(), frame.area())
}
