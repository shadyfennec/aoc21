use std::io;
use std::time::{Duration, Instant};

use color_eyre::eyre;
use crossterm::event::{self, Event, KeyCode};
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod app;
use app::*;

mod layout;
mod threadpool;
use threadpool::*;

pub fn run() -> eyre::Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let tick_rate = Duration::from_millis(16);

    let mut last_tick = Instant::now();

    let mut app = App::default();

    loop {
        terminal.draw(|f| layout::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => app.handle_key(c),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Down => app.on_down(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit() {
            break Ok(());
        }
    }
}
