use std::error;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};

// Application result type
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub running: bool,
}


// impl Default for App {
//     fn default() -> Self {
//         Self {
//             running: true
//         }
//     }
// }

/// Application
impl App {
    /// Constructs a new instance of [`App`]
    pub fn new() -> Self {
        Self {
            running: true
        }
    }

    /// Handles the tick event of the terminal
    pub fn tick(&self) {}

    /// Renders the user interface widgets
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        frame.render_widget(
            Paragraph::new("Psqlgui")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Center),
            frame.size()
        )
    }

}