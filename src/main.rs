use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use app::{App};
use editor::Editor;

mod editor;
mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let terminal_size = terminal.size()?;

    let mut app = App::new(false,
                           terminal,
                           Editor::new(
                               vec![vec![' '; terminal_size.height
                                   as usize]; terminal_size.width as usize],
                               (0, 0),
                               (0, 0),
                           ));

    loop {
        app.draw()?;
        app.update()?;
        if app.should_quit {
            break;
        }
    }

    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
