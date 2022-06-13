use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use psqlgui::app::{App, AppResult};
use psqlgui::event::{Event, EventHandler};
use psqlgui::handler::handle_key_events;
use psqlgui::tui::Tui;

fn main() -> AppResult<()> {
    /// Create an application
    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    tui.init()?;

    // Start the main looping
    while app.running {
        // Render the user interface
        tui.draw(&mut app)?;

        // Handle events
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;

    Ok(())
}