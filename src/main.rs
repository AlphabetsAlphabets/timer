use std::io::stdout;
use std::env;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{sleep, spawn};

use std::time::{Duration, SystemTime};

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::style::{Color, Colors, Print, SetColors};
use crossterm::terminal::{
    enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{cursor, execute};

use terminal_size::{terminal_size, Height, Width};

fn detect() -> Option<bool> {
    if !poll(Duration::from_millis(100)).unwrap_or_default() {
        return None;
    }

    match read().unwrap() {
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            ..
        }) => Some(true),
        _ => None,
    }
}

struct TimeDisplay(String);

// TODO: Parse time into 01h05m03s etc
impl From<String> for TimeDisplay {
    fn from(_time: String) -> Self {
        TimeDisplay("hi".to_string())
    }
}

fn main() {
    // in seconds
    let args: Vec<String> = env::args().collect();
    let message = &args[1];

    enable_raw_mode().unwrap();
    execute!(stdout(), cursor::Hide, EnterAlternateScreen).unwrap();

    // TODO: Make this more reactive
    // 5m => 05:00
    // 5m3s => 05:03
    // 1h5m3s = > 01:05:03
    let mut finish = args[2].parse::<u64>().unwrap();
    let exit = Arc::new(AtomicBool::new(false));

    let exit_signal = exit.clone();
    spawn(move || {
        while !exit_signal.load(Ordering::SeqCst) {
            if let Some(signal) = detect() {
                exit_signal.store(signal, Ordering::SeqCst);
            };
        }
    });

    while finish != 0 {
        if exit.load(Ordering::SeqCst) {
            break;
        }

        let size = terminal_size();
        let dim = if let Some((Width(w), Height(h))) = size {
            (w, h)
        } else {
            panic!("Unable to get screen dimensions");
        };

        let w = dim.0 / 2;
        let h = dim.1 / 2;

        let display = format!("{}\n", message);
        let display_length = display.len() as u16 / 2;

        let time_left = format!("{}s\n", finish);
        let left_length = time_left.len() as u16 / 2;

        let help = "Press 'q' to quit.\n".to_string();
        let help_length = help.len() as u16 / 2;

        execute!(
            stdout(),
            Clear(ClearType::All),
            SetColors(Colors::new(Color::Green, Color::Black)),
            cursor::MoveTo(w - left_length, h),
            Print(time_left),
            cursor::MoveTo(w - display_length, h - 5),
            Print(message),
            cursor::MoveTo(w - help_length, h + 3),
            Print(help)
        )
        .unwrap();

        let current = SystemTime::now();
        sleep(Duration::from_secs(1));
        let elapsed = current.elapsed().unwrap().as_secs();
        finish -= elapsed;
    }
    execute!(stdout(), LeaveAlternateScreen, cursor::Show).unwrap();
    exit.store(true, Ordering::SeqCst);
}
