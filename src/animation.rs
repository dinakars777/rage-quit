use clap::ValueEnum;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use rand::Rng;
use std::io::{self, Write};
use std::sync::atomic::{AtomicU8, Ordering};
use std::time::Duration;

static ANIMATION_SPEED: AtomicU8 = AtomicU8::new(AnimationSpeed::Normal as u8);

#[derive(Clone, Copy, Debug, ValueEnum)]
#[repr(u8)]
pub enum AnimationSpeed {
    Slow = 0,
    Normal = 1,
    Fast = 2,
}

pub fn set_speed(speed: AnimationSpeed) {
    ANIMATION_SPEED.store(speed as u8, Ordering::Relaxed);
}

pub fn scaled_duration_ms(ms: u64) -> u64 {
    match ANIMATION_SPEED.load(Ordering::Relaxed) {
        0 => ms.saturating_mul(2),
        2 => (ms / 2).max(1),
        _ => ms,
    }
}

pub fn sleep_ms(ms: u64) {
    std::thread::sleep(Duration::from_millis(scaled_duration_ms(ms)));
}

/// Print text one character at a time with a delay
pub fn typewriter(text: &str, delay_ms: u64) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for ch in text.chars() {
        let _ = execute!(handle, Print(ch));
        let _ = handle.flush();
        sleep_ms(delay_ms);
    }
}

/// Print centered text with typewriter effect
pub fn typewriter_centered(text: &str, width: u16, delay_ms: u64) {
    let padding = if text.len() < width as usize {
        (width as usize - text.len()) / 2
    } else {
        0
    };
    let padded = format!("{}{}", " ".repeat(padding), text);
    typewriter(&padded, delay_ms);
}

/// Render the rage meter animation
pub fn rage_meter(width: u16) {
    let bar_width = (width as usize).min(60);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let steps = 50;

    for i in 0..=steps {
        let pct = (i as f64 / steps as f64 * 100.0) as u8;
        let filled = (i * bar_width) / steps;
        let empty = bar_width - filled;

        let color = match pct {
            0..=30 => Color::Green,
            31..=60 => Color::Yellow,
            61..=85 => Color::Rgb {
                r: 255,
                g: 140,
                b: 0,
            }, // orange
            _ => Color::Red,
        };

        let bar = format!(
            "\r  RAGE LEVEL [{}{}] {}%",
            "█".repeat(filled),
            "░".repeat(empty),
            pct
        );

        let _ = execute!(handle, SetForegroundColor(color), Print(&bar), ResetColor);
        let _ = handle.flush();
        sleep_ms(60);
    }

    // Flash red 3 times at 100%
    for _ in 0..3 {
        let bar_full = format!("\r  RAGE LEVEL [{}] 100%", "█".repeat(bar_width));
        let _ = execute!(
            handle,
            SetForegroundColor(Color::DarkRed),
            Print(&bar_full),
            ResetColor
        );
        let _ = handle.flush();
        sleep_ms(100);
        let bar_bright = format!("\r  RAGE LEVEL [{}] 100%", "█".repeat(bar_width));
        let _ = execute!(
            handle,
            SetForegroundColor(Color::Rgb {
                r: 255,
                g: 50,
                b: 50
            }),
            Print(&bar_bright),
            ResetColor
        );
        let _ = handle.flush();
        sleep_ms(100);
    }

    println!();
}

/// Render fire animation filling screen from bottom to top
pub fn fire_animation(width: u16, height: u16) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut rng = rand::thread_rng();

    let fire_chars = ['░', '▒', '▓', '█', '▓', '▒'];
    let fire_colors = [
        Color::Rgb { r: 139, g: 0, b: 0 }, // dark red
        Color::Red,
        Color::Rgb {
            r: 255,
            g: 69,
            b: 0,
        }, // orange-red
        Color::Rgb {
            r: 255,
            g: 140,
            b: 0,
        }, // orange
        Color::Yellow,
        Color::Rgb {
            r: 255,
            g: 255,
            b: 100,
        }, // bright yellow
    ];

    let w = width as usize;
    let h = height as usize;

    // Grow fire from bottom
    for row_count in 1..=h {
        let _ = execute!(handle, crossterm::cursor::MoveTo(0, 0),);

        // Empty rows above fire
        for _ in 0..(h - row_count) {
            let _ = execute!(handle, Print(" ".repeat(w)), Print("\n"));
        }

        // Fire rows
        for y in 0..row_count {
            let intensity = y as f64 / row_count as f64; // 0 = top (coolest), 1 = bottom (hottest)
            for _ in 0..w {
                let char_idx = rng.gen_range(0..fire_chars.len());
                let color_idx = if intensity > 0.7 {
                    rng.gen_range(3..fire_colors.len())
                } else if intensity > 0.3 {
                    rng.gen_range(1..4)
                } else {
                    rng.gen_range(0..2)
                };
                let _ = execute!(
                    handle,
                    SetForegroundColor(fire_colors[color_idx]),
                    Print(fire_chars[char_idx]),
                );
            }
            let _ = execute!(handle, ResetColor, Print("\n"));
        }

        let _ = handle.flush();
        sleep_ms(40);
    }

    // Hold the full fire for a beat
    sleep_ms(300);
}

/// Render a dramatic progress bar
pub fn progress_bar(label: &str, verb: &str, size_str: &str, duration_ms: u64) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let bar_width = 30;
    let steps = 50;

    for i in 0..=steps {
        let pct = (i as f64 / steps as f64 * 100.0) as u8;
        let filled = (i * bar_width) / steps;
        let empty = bar_width - filled;

        let status = format!(
            "\r  {} {} [{}{}] {}% ({})",
            if pct < 100 { "🔥" } else { "💀" },
            verb,
            "█".repeat(filled),
            "░".repeat(empty),
            pct,
            if pct < 100 { label } else { "OBLITERATED" },
        );

        let color = if pct < 50 {
            Color::Rgb {
                r: 255,
                g: 140,
                b: 0,
            }
        } else if pct < 90 {
            Color::Red
        } else {
            Color::DarkRed
        };

        let _ = execute!(
            handle,
            SetForegroundColor(color),
            Print(&status),
            ResetColor
        );
        let _ = handle.flush();
        sleep_ms(duration_ms / steps as u64);
    }

    let _ = execute!(handle, Print(format!(" — {} freed", size_str)),);
    println!();
}

/// Format bytes into human-readable string
pub fn format_bytes(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.0} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

/// Flash fake error messages across the screen
pub fn fake_errors(width: u16, sound: Option<&crate::sound::SoundPlayer>) {
    let errors = vec![
        "✗ Destroying node_modules singularity...",
        "✗ Purging cached dependencies...",
        "✗ Detonating build artifacts...",
        "✗ Incinerating .env.local.backup.old.final(2)...",
        "✗ Formatting senior dev's hard drive... just kidding",
        "✗ Unsubscribing from all JIRA notifications...",
        "✗ Mass-deleting Slack threads about \"quick syncs\"...",
        "✗ Revoking access to the monorepo...",
        "✗ Archiving 847 unread PR reviews...",
        "✗ Ejecting from create-react-app... spiritually...",
        "✗ Dropping all production tables... in my dreams...",
        "✗ Closing 94 Chrome tabs of Stack Overflow...",
        "✗ Deleting \"fix\" commits from existence...",
        "✗ Reverting the revert of the revert...",
    ];

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut rng = rand::thread_rng();

    for msg in &errors {
        // Truncate to terminal width
        let display = fit_error_to_width(msg, width);
        let _ = execute!(
            handle,
            SetForegroundColor(Color::Red),
            Print(format!("  {}\n", display)),
            ResetColor,
        );
        let _ = handle.flush();

        // Play error sound
        if let Some(player) = sound {
            player.play(crate::sound::SoundEffect::Error);
        }

        sleep_ms(rng.gen_range(80..250));
    }
}

fn fit_error_to_width(message: &str, width: u16) -> String {
    message
        .chars()
        .take(width.saturating_sub(4) as usize)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_error_truncation_handles_narrow_widths() {
        assert_eq!(fit_error_to_width("abcdef", 0), "");
        assert_eq!(fit_error_to_width("abcdef", 3), "");
        assert_eq!(fit_error_to_width("abcdef", 6), "ab");
    }

    #[test]
    fn fake_error_truncation_preserves_utf8_boundaries() {
        assert_eq!(fit_error_to_width("ééé", 5), "é");
    }

    #[test]
    fn animation_speed_scales_durations() {
        set_speed(AnimationSpeed::Slow);
        assert_eq!(scaled_duration_ms(10), 20);

        set_speed(AnimationSpeed::Fast);
        assert_eq!(scaled_duration_ms(10), 5);
        assert_eq!(scaled_duration_ms(1), 1);

        set_speed(AnimationSpeed::Normal);
        assert_eq!(scaled_duration_ms(10), 10);
    }
}
