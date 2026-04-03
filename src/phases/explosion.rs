use std::io::{self, Write};
use std::time::Duration;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};
use crate::animation;

/// Phase 2: The explosion — fire fills the screen, fake errors cascade
pub fn run(width: u16, height: u16) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Clear for the fire
    let _ = execute!(handle, Clear(ClearType::All), MoveTo(0, 0));
    let _ = handle.flush();

    // Fire animation
    animation::fire_animation(width, height.min(20));

    std::thread::sleep(Duration::from_millis(200));

    // Clear fire, show fake errors
    let _ = execute!(handle, Clear(ClearType::All), MoveTo(0, 0));
    let _ = handle.flush();

    println!();
    animation::fake_errors(width);
    println!();

    std::thread::sleep(Duration::from_millis(500));
}
