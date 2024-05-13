// main.rs

mod bot;
mod decoder;
// mod ui;
// mod robot;
mod ux;

use crossterm::event; // provides functionality to read keyboard, mouse and terminal resize events.
use crossterm::event::DisableMouseCapture; //? https://ratatui.rs/concepts/backends/mouse-capture/
use crossterm::event::EnableMouseCapture;
use crossterm::terminal::EnterAlternateScreen; //? https://ratatui.rs/concepts/backends/alternate-screen/
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::backend::CrosstermBackend;
use std::{error::Error, io};
use tokio::runtime::Runtime; //? https://tokio.rs/tokio/tutorial/hello-tokio

fn main() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?; // create tokio runtime outside the event loop

    // ... Set up the terminal
    enable_raw_mode()?; // bypass standard input processing //? https://ratatui.rs/concepts/backends/raw-mode/
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?; // macro from the crossterm crate
    let backend = CrosstermBackend::new(stdout); // creates backend for the TUI
    let mut terminal = ratatui::Terminal::new(backend)?; // creates an instance of the TUI

    let mut input = String::new();
    let mut output = String::new();

    loop {
        // ui::draw(&mut terminal, &input, &output)?; // using tui-rs //TODO: find ui configs/customizations
        ux::draw(&mut terminal, &input, &output)?; // using ratatui //TODO: find ui configs/customizations

        // ... Handle keypress/keystroke events
        if let event::Event::Key(key) = event::read()? {
            match key.code {
                // do something if a key is pressed; for special keys, perform a defined action if key is pressed
                event::KeyCode::Enter => {
                    // if enter is pressed, process the input and call the bot
                    let input_text = input.clone(); // clone the input text
                    input.clear(); // clear the input field

                    // ...use the runtime to exec. the async function
                    let completion_result = rt.block_on(bot::generate_completion(&input_text));
                    //* get a completion from the bot w/ async; Runtime.block_on is the runtime's entry point and it runs the future arg. to completion

                    match completion_result {
                        // update the output with either the completion or the error message
                        #[rustfmt::skip]
                        Ok(completion) => { output = completion; }
                        #[rustfmt::skip]
                        Err(err) => { output = format!("Error generating completion: {}", err); }
                    }
                }
                event::KeyCode::Char(c) => input.push(c), // add typed character to the input
                #[rustfmt::skip]
                event::KeyCode::Backspace => { input.pop(); } // remove typed character from input
                #[rustfmt::skip]
                event::KeyCode::Esc => { break; } // exit the loop //TODO: confirm message or return to home; learn multipage apps
                _ => {} // catch-all for doing nothing with any other key presses //? which ones?
            }
        }
    }

    // ... Restore the terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(()) // return Ok, i.e successful completion
}
