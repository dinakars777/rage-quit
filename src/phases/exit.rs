use crate::animation;
use crate::sound::{SoundEffect, SoundPlayer};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{self, Write};

/// Phase 5: The mic drop and final exit message
pub fn run(width: u16, sound: &SoundPlayer) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    println!();

    // Center the mic drop
    let mic_art = vec![
        "     🎤     ",
        "      \\     ",
        "       \\    ",
        "        💥  ",
    ];

    for line in &mic_art {
        let padding = if (width as usize) > line.len() + 4 {
            ((width as usize) - line.len()) / 2
        } else {
            4
        };
        println!("{}{}", " ".repeat(padding), line);
        animation::sleep_ms(200);
    }

    // Mic drop sound
    sound.play(SoundEffect::MicDrop);

    println!();
    animation::sleep_ms(500);

    // Final message
    let msg = "rage-quit complete. Touch grass. 🌱";
    let padding = if (width as usize) > msg.len() + 4 {
        ((width as usize) - msg.len()) / 2
    } else {
        4
    };

    let _ = execute!(
        handle,
        SetForegroundColor(Color::Green),
        Print(format!("{}{}\n", " ".repeat(padding), msg)),
        ResetColor,
    );
    let _ = handle.flush();

    println!();
    animation::sleep_ms(500);
}
