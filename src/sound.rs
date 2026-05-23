use std::io::{self, Write};

/// Sound effect types
#[derive(Debug, Clone, Copy)]
pub enum SoundEffect {
    Bell,       // Simple terminal bell
    RapidBells, // Rapid fire bells
    Explosion,  // Bell pattern for explosion
    MicDrop,    // Dramatic bell sequence
    Error,      // Error beep
    Success,    // Success chime
}

/// Sound mode configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SoundMode {
    Silent, // No sound
    Bell,   // Terminal bell only
}

pub struct SoundPlayer {
    mode: SoundMode,
}

impl SoundPlayer {
    pub fn new(mode: SoundMode) -> Self {
        Self { mode }
    }

    /// Play a sound effect
    pub fn play(&self, effect: SoundEffect) {
        if self.mode == SoundMode::Silent {
            return;
        }

        match self.mode {
            SoundMode::Silent => {}
            SoundMode::Bell => self.play_bell(effect),
        }
    }

    /// Play terminal bell patterns
    fn play_bell(&self, effect: SoundEffect) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        match effect {
            SoundEffect::Bell => {
                let _ = write!(handle, "\x07");
                let _ = handle.flush();
            }
            SoundEffect::RapidBells => {
                for i in 0..5 {
                    let _ = write!(handle, "\x07");
                    let _ = handle.flush();
                    if i < 4 {
                        std::thread::sleep(std::time::Duration::from_millis(50));
                    }
                }
            }
            SoundEffect::Explosion => {
                // Crescendo of bells
                for i in 0..7 {
                    let _ = write!(handle, "\x07");
                    let _ = handle.flush();
                    let delay = 200 - (i * 25);
                    std::thread::sleep(std::time::Duration::from_millis(delay as u64));
                }
            }
            SoundEffect::MicDrop => {
                // Dramatic pause then two bells
                std::thread::sleep(std::time::Duration::from_millis(300));
                let _ = write!(handle, "\x07");
                let _ = handle.flush();
                std::thread::sleep(std::time::Duration::from_millis(150));
                let _ = write!(handle, "\x07");
                let _ = handle.flush();
            }
            SoundEffect::Error => {
                let _ = write!(handle, "\x07");
                let _ = handle.flush();
                std::thread::sleep(std::time::Duration::from_millis(80));
                let _ = write!(handle, "\x07");
                let _ = handle.flush();
            }
            SoundEffect::Success => {
                // Rising tone pattern
                for _ in 0..3 {
                    let _ = write!(handle, "\x07");
                    let _ = handle.flush();
                    std::thread::sleep(std::time::Duration::from_millis(120));
                }
            }
        }
    }
}

/// Create a sound player based on CLI flags
pub fn create_player(silent: bool, sound: bool, bell_only: bool) -> SoundPlayer {
    let mode = if silent {
        SoundMode::Silent
    } else if sound || bell_only {
        SoundMode::Bell
    } else {
        // Default: use bells
        SoundMode::Bell
    };

    SoundPlayer::new(mode)
}
