use app::App;
use crossterm;
use crossterm::ExecutableCommand;
use editor::Editor;
use std::io;

mod app;
mod editor;

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    io::stdout()
        .execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))
        .unwrap();
    io::stdout().execute(crossterm::cursor::Hide).unwrap();

    let terminal_size = crossterm::terminal::size().unwrap();

    let mut app = App::new();

    app.run();

    crossterm::terminal::disable_raw_mode().unwrap();
    io::stdout().execute(crossterm::cursor::Show).unwrap();
}
