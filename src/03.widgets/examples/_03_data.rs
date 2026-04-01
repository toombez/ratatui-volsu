use ratatui::style::{Style, Stylize};
use ratatui::{DefaultTerminal, Frame, symbols};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Axis, Bar, BarChart, BarGroup, Block, Chart, Dataset, GraphType};

pub fn example_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;

        if crossterm::event::read()?.is_key_press() {
            break Ok(());
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
        let datasets = vec![
            Dataset::default()
                .name("data1")
                .marker(symbols::Marker::Dot)
                .graph_type(GraphType::Scatter)
                .style(Style::default().cyan())
                .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
            Dataset::default()
                .name("data2")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().magenta())
                .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
        ];

        let x_axis = Axis::default()
            .title("X Axis".red())
            .style(Style::default().white())
            .bounds([0.0, 10.0])
            .labels(["0.0", "5.0", "10.0"]);

        let y_axis = Axis::default()
            .title("Y Axis".red())
            .style(Style::default().white())
            .bounds([0.0, 10.0])
            .labels(["0.0", "5.0", "10.0"]);

        let chart = Chart::new(datasets)
            .block(Block::new().title("Chart"))
            .x_axis(x_axis)
            .y_axis(y_axis);

        frame.render_widget(chart, layout[0]);
    }

    {
        let bar_chart = BarChart::default()
            .block(Block::bordered().title("BarChart"))
            .bar_width(3)
            .bar_gap(1)
            .group_gap(3)
            .bar_style(Style::new().yellow().on_red())
            .value_style(Style::new().red().bold())
            .label_style(Style::new().white())
            .data(&[("A0", 0), ("A1", 2), ("A2", 4), ("A3", 3)])
            .data(BarGroup::new([
                Bar::with_label("B0", 10),
                Bar::with_label("B2", 20),
            ]))
            .max(4);

        frame.render_widget(bar_chart, layout[1]);
    }
}
