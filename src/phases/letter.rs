use crate::analyzer::ProjectStats;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::time::Duration;

const LETTER_BOX_WIDTH: usize = 58;

/// Phase 4: The resignation letter — dynamically generated from project stats
pub fn run(stats: &ProjectStats, width: u16) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let _ = execute!(handle, Clear(ClearType::All), MoveTo(0, 0));
    let _ = handle.flush();

    let letter = generate_letter(stats);
    let padding = if (width as usize) > LETTER_BOX_WIDTH + 4 {
        ((width as usize) - LETTER_BOX_WIDTH) / 2
    } else {
        2
    };
    let pad = " ".repeat(padding);

    println!();
    std::thread::sleep(Duration::from_millis(300));

    // Draw the box
    let _ = execute!(
        handle,
        SetForegroundColor(Color::Rgb {
            r: 200,
            g: 200,
            b: 200
        })
    );

    // Top border
    println!("{}┌{}┐", pad, "─".repeat(LETTER_BOX_WIDTH));

    // Title
    let title = "LETTER OF RESIGNATION";
    let title_pad = (LETTER_BOX_WIDTH - title.chars().count()) / 2;
    println!(
        "{}│{}{}{}│",
        pad,
        " ".repeat(title_pad),
        title,
        " ".repeat(LETTER_BOX_WIDTH - title_pad - title.chars().count())
    );
    println!("{}│{}│", pad, " ".repeat(LETTER_BOX_WIDTH));

    // Letter body
    for line in &letter {
        let display_line = fit_letter_line(line, LETTER_BOX_WIDTH);
        let right_pad = letter_right_padding(&display_line, LETTER_BOX_WIDTH);
        println!(
            "{}│ {}{} │",
            pad,
            display_line,
            " ".repeat(right_pad.max(0))
        );
        std::thread::sleep(Duration::from_millis(80));
    }

    // Bottom border
    println!("{}└{}┘", pad, "─".repeat(LETTER_BOX_WIDTH));

    let _ = execute!(handle, ResetColor);
    println!();
    std::thread::sleep(Duration::from_millis(1500));
}

fn fit_letter_line(line: &str, box_width: usize) -> String {
    let content_width = box_width.saturating_sub(4);
    if line.chars().count() <= content_width {
        return line.to_string();
    }

    let prefix: String = line.chars().take(content_width.saturating_sub(3)).collect();
    format!("{prefix}...")
}

fn letter_right_padding(line: &str, box_width: usize) -> usize {
    box_width.saturating_sub(2 + line.chars().count())
}

/// Generate the letter content from project stats
pub fn generate_letter(stats: &ProjectStats) -> Vec<String> {
    let mut lines = Vec::new();

    // Salutation
    lines.push(format!("Dear {},", stats.project_name));
    lines.push(String::new());

    // Opening line
    if let (Some(commits), Some(days)) = (stats.git_commits, stats.git_age_days) {
        let age = humanize_days(days);
        lines.push(format!("After {} commits spanning {}, I", commits, age));
        lines.push("can no longer pretend this codebase".to_string());
        lines.push("sparks joy.".to_string());
    } else {
        lines.push("I regret to inform you that I am".to_string());
        lines.push("leaving this project, effective".to_string());
        lines.push("immediately.".to_string());
    }
    lines.push(String::new());

    // Grievances
    lines.push("Notable grievances:".to_string());
    let grievances = generate_grievances(stats);
    for g in &grievances {
        lines.push(format!("• {}", g));
    }
    lines.push(String::new());

    // Sign-off
    let signoffs = [
        vec![
            "I wish you nothing but segfaults.".to_string(),
            String::new(),
            "Regards,".to_string(),
            "A dev who tried".to_string(),
        ],
        vec![
            "With zero regrets,".to_string(),
            "The last person who understood".to_string(),
            "this codebase".to_string(),
        ],
        vec![
            "May your builds be green.".to_string(),
            "(They won't be.)".to_string(),
        ],
        vec![
            "Sincerely done,".to_string(),
            "A developer who committed".to_string(),
            "to the wrong branch of life".to_string(),
        ],
    ];

    let mut rng = rand::thread_rng();
    if let Some(signoff) = signoffs.choose(&mut rng) {
        for line in signoff {
            lines.push(line.clone());
        }
    }

    lines.push(String::new());
    lines.push(format!("Date: {}", chrono::Local::now().format("%Y-%m-%d")));

    lines
}

fn generate_grievances(stats: &ProjectStats) -> Vec<String> {
    let mut grievances = Vec::new();

    if stats.utils_file_count > 3 {
        grievances.push(format!(
            "{} files named 'utils'. Peak creativity.",
            stats.utils_file_count
        ));
    } else if stats.utils_file_count > 0 {
        grievances.push(format!(
            "{} file(s) named 'utils' — original.",
            stats.utils_file_count
        ));
    }

    if stats.dependency_count > 150 {
        grievances.push(format!(
            "{} dependencies. This isn't a project,",
            stats.dependency_count
        ));
        grievances.push("  it's a support group.".to_string());
    } else if stats.dependency_count > 50 {
        grievances.push("A dependency list longer than my will".to_string());
        grievances.push(format!("  to live ({} packages).", stats.dependency_count));
    } else if stats.dependency_count > 0 {
        grievances.push(format!(
            "{} dependencies — each one a tiny regret.",
            stats.dependency_count
        ));
    }

    if stats.total_files > 0 {
        grievances.push(format!(
            "{} {} files, somehow still not enough",
            stats.total_files, stats.project_type
        ));
        grievances.push("  abstraction.".to_string());
    }

    if let Some((ref name, lines)) = stats.largest_file {
        if lines > 500 {
            grievances.push(format!("A {}-line file called '{}'. It has", lines, name));
            grievances.push("  its own weather system.".to_string());
        } else if lines > 200 {
            grievances.push(format!(
                "'{}' at {} lines — nobody dares scroll",
                name, lines
            ));
            grievances.push("  to the bottom.".to_string());
        }
    }

    if stats.todo_count > 20 {
        grievances.push(format!(
            "{} TODOs that will never be done.",
            stats.todo_count
        ));
        grievances.push("  They're not TODOs, they're prayers.".to_string());
    } else if stats.todo_count > 0 {
        grievances.push(format!(
            "{} optimistic TODOs from a younger,",
            stats.todo_count
        ));
        grievances.push("  more innocent developer.".to_string());
    }

    if stats.total_bloat_bytes > 524_288_000 {
        let size = crate::animation::format_bytes(stats.total_bloat_bytes);
        grievances.push(format!("Bloat weighing {}. Heavier than my", size));
        grievances.push("  existential dread.".to_string());
    }

    if let Some(commits) = stats.git_commits {
        if commits > 500 {
            grievances.push(format!("{} commits. Stockholm syndrome", commits));
            grievances.push("  at its finest.".to_string());
        }
    }

    // Fallbacks to always have at least 3
    let fallbacks = vec![
        "That one function everyone's afraid\n  to touch.".to_string(),
        "Code that works. Nobody knows why.".to_string(),
        "A README that says 'TODO: write\n  README'.".to_string(),
        "The config file that is actually\n  load-bearing.".to_string(),
    ];

    let mut rng = rand::thread_rng();
    let mut fallback_iter = fallbacks.into_iter().collect::<Vec<_>>();
    fallback_iter.shuffle(&mut rng);

    while grievances.len() < 3 {
        if let Some(f) = fallback_iter.pop() {
            // Split multiline fallbacks
            for part in f.split('\n') {
                grievances.push(part.to_string());
            }
        } else {
            break;
        }
    }

    // Cap at ~5 visible grievances (not counting continuation lines)
    grievances.truncate(10);
    grievances
}

fn humanize_days(days: u64) -> String {
    if days > 730 {
        format!("{} years", days / 365)
    } else if days > 365 {
        "over a year".to_string()
    } else if days > 60 {
        format!("{} months", days / 30)
    } else if days > 1 {
        format!("{} days", days)
    } else {
        "less than a day".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_line_truncation_preserves_utf8_boundaries() {
        let line = format!("{}—{}", "a".repeat(60), "done");
        let fitted = fit_letter_line(&line, LETTER_BOX_WIDTH);

        assert!(fitted.ends_with("..."));
        assert!(fitted.is_char_boundary(fitted.len()));
        assert!(fitted.chars().count() <= LETTER_BOX_WIDTH - 4);
    }

    #[test]
    fn letter_padding_saturates_for_tiny_boxes() {
        assert_eq!(letter_right_padding("long line", 4), 0);
    }
}
