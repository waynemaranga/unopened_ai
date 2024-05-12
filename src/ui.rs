// ui.rs

// use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use std::io;
use tui::{
    backend::Backend,
    // backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, input: &str, output: &str) -> io::Result<()> {
    // Return an io::Result<()>
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
            .split(f.size());

        let input_widget = Paragraph::new(input.to_string())
            .block(Block::default().title("Input").borders(Borders::ALL));
        f.render_widget(input_widget, chunks[0]);

        let output_widget = Paragraph::new(Spans::from(vec![
            Span::styled(
                "Output: ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(output),
        ]))
        .wrap(Wrap { trim: true })
        .block(Block::default().title("Output").borders(Borders::ALL));
        f.render_widget(output_widget, chunks[1]);
    })?; // Note the '?' for error propagation

    Ok(()) // Explicitly return Ok(())
}
