// ux.rs
//! not user experience; TUI, using `ratatui` (`ui.rs` uses `tui-rs`)
//!
//! Ratatui docs: https://ratatui.rs/how-to/

use chrono::Local;
use ratatui::{
    backend::Backend,                        //? https://ratatui.rs/concepts/backends/
    layout::{Constraint, Direction, Layout}, //? https://ratatui.rs/concepts/layout/
    style::{Color, Modifier, Style},
    text::{Line, Span, Text}, // key change from tui; ratatui does't have Spans
    widgets::{Block, Borders, Paragraph, Wrap}, //? https://ratatui.rs/concepts/widgets/
    Terminal,
};
use std::io;

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, input: &str, output: &str) -> io::Result<()> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
            .split(f.size());

        // TODO: try using the Rect from Ratatui: https://ratatui.rs/how-to/layout/center-a-rect/

        // --- Date & Time Display ---
        let now = Local::now();
        let datetime_string = now.format("%Y-%m-%d %H:%M:%S").to_string();

        let datetime_widget = Paragraph::new(datetime_string)
            .style(Style::default().fg(Color::Yellow)) // Customize color
            .block(Block::default().borders(Borders::NONE));
        f.render_widget(datetime_widget, chunks[0]); // Render at the top left

        // --- Input Widget ---
        let input_widget = Paragraph::new(input.to_string())
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::ITALIC),
            ) // Italic input
            .block(Block::default().title("Input").borders(Borders::ALL));
        f.render_widget(input_widget, chunks[0]);

        // --- Output Widget ---
        // let output_widget = Paragraph::new(Spans::from(vec![ //* tui has no Spans from a vector of Span objects, just a vector of Span objects directly
        // let output_widget = Paragraph::new(vec![
        let output_text = Text::from(vec![Line::from(vec![
            Span::styled(
                "Output: ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD), // Bold "Output:"
            ),
            Span::raw(output),
        ])]);

        let output_widget = Paragraph::new(output_text)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White))
            .block(Block::default().title("Output").borders(Borders::ALL));

        // .wrap(Wrap { trim: true })
        // .style(Style::default().fg(Color::White)) // Output text in white
        // .block(Block::default().title("Output").borders(Borders::ALL));

        f.render_widget(output_widget, chunks[1]); // TODO: make layout dynamic https://ratatui.rs/how-to/layout/dynamic/
    })?; //TODO: review error propagation and explicit return of Result

    Ok(())
}
