// main.rs

mod bot;
mod ui; // Declare the ui module

use crossterm::event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{error::Error, io};
use tokio::runtime::Runtime;
use tui::backend::CrosstermBackend;

fn main() -> Result<(), Box<dyn Error>> {
    // ... create tokio runtime outside the event loop
    let rt = Runtime::new()?;

    // ... set up the terminal
    enable_raw_mode()?; //? what's raw mode?
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;

    let mut input = String::new();
    let mut output = String::new();

    /* -- Old Event loop
        loop {
            ui::draw(&mut terminal, &input, &output)?; // Draw UI

            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Enter => {
                        // Process input and update output
                        output = process_input(&input); // Replace with your logic
                        input.clear();
                    }
                    event::KeyCode::Char(c) => input.push(c),
                    event::KeyCode::Backspace => {
                        input.pop();
                    }
                    event::KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    */

    loop {
        ui::draw(&mut terminal, &input, &output)?; // draw the ui //TODO: find ui configs/customizations

        if let event::Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Enter => {
                    let input_text = input.clone();
                    input.clear();

                    // ... use the runtime to exec. the async function
                    let completion_result = rt.block_on(bot::generate_completion(&input_text));

                    match completion_result {
                        Ok(completion) => {
                            output = completion;
                        }
                        Err(err) => {
                            output = format!("Error generating completion: {}", err);
                        }
                    }
                }
                event::KeyCode::Char(c) => input.push(c),
                event::KeyCode::Backspace => {
                    input.pop();
                }
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

// Placeholder function for input processing (replace with your own logic)
// fn process_input(input: &str) -> String {
//     format!("You entered: {}", input)
// }
