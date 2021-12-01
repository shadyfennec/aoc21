use color_eyre::eyre;

use tui::Terminal;

mod app;
use app::*;

mod layout;
mod threadpool;
use threadpool::*;

#[cfg(target_os = "linux")]
mod implementation {
    use super::*;

    use std::{io, sync::mpsc, thread, time::Duration};
    use termion::{
        event::Key,
        input::{MouseTerminal, TermRead},
        raw::IntoRawMode,
        screen::AlternateScreen,
    };
    use tui::backend::TermionBackend;

    enum Event {
        Input(Key),
        Tick,
    }

    fn events(tick_rate: Duration) -> mpsc::Receiver<Event> {
        let (tx, rx) = mpsc::channel();
        let keys_tx = tx.clone();
        thread::spawn(move || {
            let stdin = io::stdin();
            stdin.keys().for_each(|evt| {
                if let Ok(key) = evt {
                    if let Err(err) = keys_tx.send(Event::Input(key)) {
                        eprintln!("{}", err);
                    }
                }
            });
        });
        thread::spawn(move || loop {
            if let Err(err) = tx.send(Event::Tick) {
                eprintln!("{}", err);
                break;
            }
            thread::sleep(tick_rate);
        });
        rx
    }

    pub fn run() -> eyre::Result<()> {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;

        let tick_rate = Duration::from_millis(16);

        let mut app = App::default();

        let events = events(tick_rate);

        loop {
            terminal.draw(|f| layout::draw(f, &mut app))?;

            match events.recv()? {
                Event::Input(key) => match key {
                    Key::Char('\n') => app.on_enter(),
                    Key::Char(c) => app.handle_key(c),
                    Key::Up => app.on_up(),
                    Key::Down => app.on_down(),
                    _ => {}
                },
                Event::Tick => app.on_tick(),
            }

            if app.should_quit() {
                break Ok(());
            }
        }
    }
}

#[cfg(not(target_os = "linux"))]

mod implementation {
    use super::*;
    use crossterm::event::{self, Event, KeyCode};
    use tui::backend::CrosstermBackend;

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
                        KeyCode::Enter => app.on_enter(),
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
}

pub use implementation::*;
