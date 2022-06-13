use std::io;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::Backend;
use tui::Terminal;
use crate::app::{App, AppResult};
use crate::event::EventHandler;

#[derive(Debug)]
pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    pub events: EventHandler,
}

impl<B:Backend> Tui<B> {
    /// Constructs a instance on [`Tui`]
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self {
            terminal,
            events
        }
    }

    /// initializes the terminal interface
    /// It enable the raw mode and sets terminal properties
    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode();
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    /// [`Draw`] the terminal interface [`rendering`] the widget
    /// [`Draw`]: tui::Terminal::draw
    /// [`rendering`]: crate::app::App::render
    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal.draw(|frame| app.render(frame))?;
        Ok(())
    }

    /// Exits the terminal interface
    /// It disables the raw mode and reverts back the terminal properties
    pub fn exit(&mut self) -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        self.terminal.show_cursor()?;
        Ok(())
    }

}