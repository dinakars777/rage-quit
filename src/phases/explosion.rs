use crate::animation;
use crate::sound::{SoundEffect, SoundPlayer};
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

/// Phase 2: The explosion — fire fills the screen, fake errors cascade
pub fn run(width: u16, height: u16, sound: &SoundPlayer) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Clear for the fire
    let _ = execute!(handle, Clear(ClearType::All), MoveTo(0, 0));
    let _ = handle.flush();

    // Explosion sound
    sound.play(SoundEffect::Explosion);

    // Fire animation
    animation::fire_animation(width, height.min(20));

    animation::sleep_ms(200);

    // Clear fire, show fake errors
    let _ = execute!(handle, Clear(ClearType::All), MoveTo(0, 0));
    let _ = handle.flush();

    println!();
    animation::fake_errors(width, Some(sound));
    println!();

    animation::sleep_ms(500);
}
