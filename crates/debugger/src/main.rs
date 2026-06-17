use std::sync::{Arc, Mutex};
use std::time::Duration;

use clap::Parser;
use crossterm::{
    event::{Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{Terminal, backend::CrosstermBackend};

mod app;
mod input;
mod state;
mod ui;
mod ui_state;

use app::Debugger;
use std::io::Write;
use tracing_subscriber::fmt::MakeWriter;

#[derive(Clone)]
struct TuiWriter {
    logs: Arc<Mutex<Vec<state::ConsoleEntry>>>,
}

impl Write for TuiWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let msg = String::from_utf8_lossy(buf).to_string();
        if let Ok(mut logs) = self.logs.lock() {
            let level = if msg.contains("ERROR") {
                state::ConsoleLevel::Error
            } else if msg.contains("WARN") {
                state::ConsoleLevel::Warn
            } else {
                state::ConsoleLevel::Info
            };
            logs.push(state::ConsoleEntry {
                message: msg.trim_end().to_string(),
                level,
                tick: 0,
            });
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for TuiWriter {
    type Writer = Self;
    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

#[derive(Parser, Debug)]
#[command(version, about = "Xoloria interactive debugger")]
struct Opts {
    #[arg(short)]
    binary: String,

    #[arg(short, long)]
    elf: Option<String>,
}

thread_local! {
    pub static SUPPRESS_PANIC_HOOK: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let dbg_instance = Debugger::new(&opts.binary, opts.elf.as_deref())?;
    let tracing_log = Arc::clone(&dbg_instance.tracing_log);
    let dbg = Arc::new(Mutex::new(dbg_instance));

    let writer = TuiWriter { logs: tracing_log };
    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_ansi(false)
        .init();

    {
        let original_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if SUPPRESS_PANIC_HOOK.with(|flag| flag.get()) {
                return;
            }

            let _ = crossterm::terminal::disable_raw_mode();
            let _ =
                crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen);
            original_hook(info);
        }));
    }

    crossterm::terminal::enable_raw_mode()?;

    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = {
        let mut dbg = dbg.lock().expect("Failed to lock debugger");
        run_loop(&mut terminal, &mut dbg)
    };

    crossterm::terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    dbg: &mut Debugger,
) -> anyhow::Result<()> {
    while dbg.running {
        terminal.draw(|frame| dbg.render(frame))?;

        if crossterm::event::poll(match dbg.hart_modes.contains(&state::HartMode::Running) {
            true => Duration::from_millis(16),
            false => Duration::from_millis(200),
        })? {
            let event = crossterm::event::read()?;
            match event {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    dbg.handle_key(key);
                }
                Event::Mouse(mouse) => {
                    dbg.handle_mouse(mouse);
                }
                _ => {}
            }
        }

        dbg.tick();
    }

    Ok(())
}
