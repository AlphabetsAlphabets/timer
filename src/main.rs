use std::env;
use std::fmt;
use std::io::stdout;

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

struct Time(usize, usize, usize);

impl From<String> for Time {
    fn from(s: String) -> Self {
        let s = s.split(':');
        let s: Vec<usize> = s.rev().take(3).map(|x| x.parse().unwrap_or(0)).collect();
        Time(
            *s.get(2).unwrap_or(&0),
            *s.get(1).unwrap_or(&0),
            *s.get(0).unwrap_or(&0),
        )
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}h{}m{}s\nIf it's all 0's separate it by hours:minute:seconds",
            self.0, self.1, self.2
        )
    }
}

fn main() {
    // in seconds
    let args: Vec<String> = env::args().collect();
    let message = args[1].to_owned();

    let time = args[2].to_owned();
    let time = Time::from(time);
    let mut total = time.0 * 3600 + time.1 * 60 + time.0;

    const SECONDS_IN_HOUR: usize = 3600;
    const SECONDS_IN_MINUTES: usize = 60;

    enable_raw_mode().unwrap();
    execute!(stdout(), cursor::Hide, EnterAlternateScreen).unwrap();

    let exit = Arc::new(AtomicBool::new(false));
    let exit_signal = exit.clone();
    spawn(move || {
        while !exit_signal.load(Ordering::SeqCst) {
            if let Some(signal) = detect() {
                exit_signal.store(signal, Ordering::SeqCst);
            };
        }
    });

    let original = total;

    while total != 0 {
        if exit.load(Ordering::SeqCst) {
            break;
        }

        let hours = total / SECONDS_IN_HOUR;
        let mut remaining_seconds = total - hours * SECONDS_IN_HOUR;
        let minutes = remaining_seconds / SECONDS_IN_MINUTES;

        remaining_seconds = remaining_seconds - minutes * SECONDS_IN_MINUTES;
        let seconds = remaining_seconds;

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

        let time_left = format!("{}h {}m {}s", hours, minutes, seconds);
        let left_length = time_left.len() as u16 / 2;

        let help = "Press 'q' to quit.\n".to_string();
        let help_length = help.len() as u16 / 2;

        let color = {
            if total >= original * 5 / 10 {
                Colors::new(Color::Green, Color::Black)
            } else if total <= original * 5 / 10 {
                Colors::new(Color::Yellow, Color::Black)
            } else if total <= original * 3 / 10 {
                Colors::new(Color::Red, Color::Black)
            } else {
                Colors::new(Color::DarkRed, Color::Black)
            }
        };

        execute!(
            stdout(),
            Clear(ClearType::All),
            SetColors(color),
            // Moves the display message to the top center if the screen.
            cursor::MoveTo(w - left_length, h),
            Print(time_left),
            // Moves to center
            SetColors(Colors::new(Color::Green, Color::Black)),
            cursor::MoveTo(w - display_length, h - 5),
            Print(display),
            // Moves to bottom center
            cursor::MoveTo(w - help_length, h + 3),
            Print(help)
        )
        .unwrap();

        let current = SystemTime::now();
        sleep(Duration::from_secs(1));
        let elapsed = current.elapsed().unwrap().as_secs() as usize;
        total -= elapsed;
    }
    execute!(stdout(), LeaveAlternateScreen, cursor::Show).unwrap();
    exit.store(true, Ordering::SeqCst);
}
