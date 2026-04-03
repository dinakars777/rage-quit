use std::io::{self, Write};
use std::time::Duration;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
    style::{Color, SetForegroundColor, ResetColor},
};
use crate::animation;

/// Phase 1: The dramatic buildup — screen goes dark, typewriter text, rage meter fills
pub fn run(width: u16, height: u16) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Clear screen
    let _ = execute!(handle, Clear(ClearType::All), MoveTo(0, 0));
    let _ = handle.flush();

    std::thread::sleep(Duration::from_millis(800));

    // Move to roughly center of screen
    let center_y = height / 3;
    let _ = execute!(handle, MoveTo(0, center_y));

    // Dramatic opening
    let _ = execute!(
        handle,
        SetForegroundColor(Color::Rgb { r: 180, g: 180, b: 180 }),
    );
    animation::typewriter_centered("You have mass-committed enough.", width, 50);
    println!();
    std::thread::sleep(Duration::from_millis(400));

    animation::typewriter_centered("It's time to go.", width, 70);
    let _ = execute!(handle, ResetColor);
    println!();
    println!();

    std::thread::sleep(Duration::from_millis(600));

    // Rage meter
    animation::rage_meter(width);
    println!();

    std::thread::sleep(Duration::from_millis(300));
}
