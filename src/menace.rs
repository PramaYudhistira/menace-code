// Main entry point for the Menace chat application.
//
// This file handles:
// - Terminal setup and teardown
// - Main event loop coordination
// - Module orchestration

mod agents;
mod config;
mod core;
mod handlers;
mod types;
mod ui;

use std::io;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::CrosstermBackend, Terminal, TerminalOptions, Viewport};

use core::App;
use handlers::{handle_events, AppEvent};

fn main() -> Result<(), io::Error> {
    // Setup terminal WITHOUT alternate screen - inline for natural scrolling
    enable_raw_mode()?;
    let stdout = io::stdout();
    // NO mouse capture - let terminal handle scrolling naturally
    let backend = CrosstermBackend::new(stdout);

    // Inline viewport - messages print to terminal, input stays at bottom
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(4), // Just for input area
        }
    )?;

    // Create app and run
    let mut app = App::default();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    terminal.show_cursor()?;

    // Handle errors
    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    let mut printed_count = 0;

    // Initial draw ONCE
    terminal.draw(|f| ui::draw(f, app))?;

    loop {
        // Handle events (this BLOCKS until an event happens - no polling!)
        match handle_events(app)? {
            AppEvent::Quit => return Ok(()),

            AppEvent::InputChanged => {
                // Redraw only the input area
                terminal.draw(|f| ui::draw(f, app))?;
            }

            AppEvent::MessageSent => {
                // Print new messages above input
                while printed_count < app.messages.len() {
                    let msg = &app.messages[printed_count];
                    use crate::types::MessageRole;
                    use ratatui::{
                        style::{Color, Style},
                        text::{Line, Span},
                        widgets::{Paragraph, Widget},
                    };

                    let (prefix, style) = match msg.role {
                        MessageRole::User => ("You: ", Style::default().fg(Color::Cyan)),
                        MessageRole::Assistant => ("AI: ", Style::default().fg(Color::Green)),
                        MessageRole::System => ("System: ", Style::default().fg(Color::Yellow)),
                    };

                    // Insert message above input
                    terminal.insert_before(1, |buf| {
                        Paragraph::new(Line::from(vec![
                            Span::styled(prefix, style),
                            Span::raw(&msg.content),
                        ]))
                        .render(buf.area, buf);
                    })?;

                    // Spacing
                    terminal.insert_before(1, |buf| {
                        Paragraph::new("").render(buf.area, buf);
                    })?;

                    printed_count += 1;
                }

                // Redraw to clear the input
                terminal.draw(|f| ui::draw(f, app))?;
            }

            AppEvent::Continue => {
                // Do nothing - no redraw needed!
            }
        }
    }
}