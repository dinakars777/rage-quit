use crate::analyzer::ProjectStats;
use crate::animation;
use crate::sound::{SoundEffect, SoundPlayer};
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, Read, Write};

/// Phase 3: Cleanup — progress bars for each bloat dir, optionally nuke them
pub fn run(stats: &ProjectStats, nuke: bool, sound: &SoundPlayer) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let _ = execute!(handle, Clear(ClearType::All), MoveTo(0, 0));
    let _ = handle.flush();

    if stats.bloat_dirs.is_empty() {
        println!();
        let _ = execute!(
            handle,
            SetForegroundColor(Color::Yellow),
            Print("  🔍 No bloat directories detected. This project is suspiciously clean.\n"),
            ResetColor,
        );
        println!();
        animation::sleep_ms(1500);
        return;
    }

    println!();
    if nuke {
        let _ = execute!(
            handle,
            SetForegroundColor(Color::Red),
            SetAttribute(Attribute::Bold),
            Print("  ☢️  NUKE MODE ENGAGED — PREPARING TO ANNIHILATE:\n"),
            ResetColor,
        );

        for dir in &stats.bloat_dirs {
            let _ = execute!(
                handle,
                SetForegroundColor(Color::DarkRed),
                Print(format!(
                    "    • {} ({})\n",
                    dir.label,
                    animation::format_bytes(dir.size_bytes)
                )),
                ResetColor,
            );
        }

        println!();
        let total = animation::format_bytes(stats.total_bloat_bytes);
        let _ = execute!(
            handle,
            SetForegroundColor(Color::Red),
            Print(format!("  Total: {}. Proceed? [y/N] ", total)),
            ResetColor,
        );
        let _ = handle.flush();

        // Read single character
        let mut buf = [0u8; 1];
        let confirmed = if crossterm::terminal::is_raw_mode_enabled().unwrap_or(false) {
            // In raw mode, read directly
            if io::stdin().read_exact(&mut buf).is_ok() {
                buf[0] == b'y' || buf[0] == b'Y'
            } else {
                false
            }
        } else {
            // Not in raw mode, read a line
            let mut input = String::new();
            io::stdin().read_line(&mut input).ok();
            input.trim().eq_ignore_ascii_case("y")
        };

        if !confirmed {
            println!();
            let _ = execute!(
                handle,
                SetForegroundColor(Color::Yellow),
                Print("  💛 Wise choice. Running in dry-run mode instead.\n"),
                ResetColor,
            );
            println!();
            run_progress_bars(stats, false, sound);
            return;
        }

        println!();
        run_progress_bars(stats, true, sound);
    } else {
        let _ = execute!(
            handle,
            SetForegroundColor(Color::Yellow),
            SetAttribute(Attribute::Bold),
            Print("  🔥 DRY RUN — SIMULATING DESTRUCTION:\n"),
            ResetColor,
        );
        println!();
        run_progress_bars(stats, false, sound);
    }

    // Summary
    println!();
    let total = animation::format_bytes(stats.total_bloat_bytes);
    if nuke {
        let _ = execute!(
            handle,
            SetForegroundColor(Color::Rgb {
                r: 255,
                g: 100,
                b: 100
            }),
            Print(format!(
                "  💥 Reclaimed {} of disk space. Freedom.\n",
                total
            )),
            ResetColor,
        );
    } else {
        let _ = execute!(
            handle,
            SetForegroundColor(Color::Yellow),
            Print(format!(
                "  💭 Would have freed {}. Use --nuke to make it real.\n",
                total
            )),
            ResetColor,
        );
    }

    println!();
    animation::sleep_ms(800);
}

fn run_progress_bars(stats: &ProjectStats, actually_delete: bool, sound: &SoundPlayer) {
    for dir in &stats.bloat_dirs {
        let size_str = animation::format_bytes(dir.size_bytes);
        let label = if actually_delete {
            dir.label.clone()
        } else {
            format!("{} [DRY RUN]", dir.label)
        };

        animation::progress_bar(&label, &dir.destruction_verb, &size_str, 1200);

        if actually_delete {
            let _ = std::fs::remove_dir_all(&dir.path);
            sound.play(SoundEffect::Success);
        } else {
            sound.play(SoundEffect::Bell);
        }
    }
}
