// ui.rs
//! Terminal ui, using tui-rs (ux.rs uses ratatui)

use chrono::{DateTime, Local}; // For date and time
use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, input: &str, output: &str) -> io::Result<()> {
    terminal.draw(|f: &mut tui::Frame<'_, B>| {
        let chunks: Vec<tui::layout::Rect> = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
            .split(f.size());

        // --- Date & Time Display ---
        let now: DateTime<Local> = Local::now();
        let datetime_string: String = now.format("%Y-%m-%d %H:%M:%S").to_string();

        let datetime_widget: Paragraph<'_> = Paragraph::new(datetime_string)
            .style(Style::default().fg(Color::Yellow)) // Customize color
            .block(Block::default().borders(Borders::NONE));
        f.render_widget(datetime_widget, chunks[0]); // Render at the top left

        // --- Input Widget ---
        let input_widget: Paragraph<'_> = Paragraph::new(input.to_string())
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::ITALIC),
            ) // Italic input
            .block(Block::default().title("Input").borders(Borders::ALL));
        f.render_widget(input_widget, chunks[0]);

        // --- Output Widget ---
        let output_widget: Paragraph<'_> = Paragraph::new(Spans::from(vec![
            Span::styled(
                "Output: ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD), // Bold "Output:"
            ),
            Span::raw(output),
        ]))
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White)) // Output text in white
        .block(Block::default().title("Output").borders(Borders::ALL));
        f.render_widget(output_widget, chunks[1]);
    })?; //TODO: review error propagation and explicit return of Result

    Ok(())
}
