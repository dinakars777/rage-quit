use crate::analyzer::ProjectType;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// Handle --comeback: reinstall dependencies with a dramatic crawling-back animation
pub fn run(target: &PathBuf) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let _ = execute!(handle, Clear(ClearType::All), MoveTo(0, 0));
    let _ = handle.flush();

    println!();

    // Crawling back animation
    let frames = vec![
        ("  _o/     ", "Wait... I didn't mean it..."),
        ("  _/o\\    ", "The job market is rough..."),
        ("  /o\\_    ", "I actually like this codebase..."),
        ("  _o/     ", "Please take me back..."),
    ];

    for (art, msg) in &frames {
        let _ = execute!(
            handle,
            SetForegroundColor(Color::Yellow),
            Print(format!("\r  {}  {}", art, msg)),
            ResetColor,
        );
        let _ = handle.flush();
        std::thread::sleep(Duration::from_millis(800));
        let _ = execute!(handle, Print("\n"));
    }

    println!();
    std::thread::sleep(Duration::from_millis(500));

    // Detect project type and reinstall
    let project_type = detect_type(target);

    let (cmd, args, label) = match project_type {
        ProjectType::Node => ("npm", vec!["install"], "npm install"),
        ProjectType::Rust => ("cargo", vec!["build"], "cargo build"),
        ProjectType::Python => (
            "pip",
            vec!["install", "-r", "requirements.txt"],
            "pip install",
        ),
        ProjectType::Go => ("go", vec!["mod", "download"], "go mod download"),
        ProjectType::Unknown => {
            let _ = execute!(
                handle,
                SetForegroundColor(Color::Yellow),
                Print("  🤷 Can't detect project type. You're on your own.\n"),
                ResetColor
            );
            return;
        }
    };

    let _ = execute!(
        handle,
        SetForegroundColor(Color::Cyan),
        Print(format!("  📦 Running {}...\n\n", label)),
        ResetColor,
    );
    let _ = handle.flush();

    let status = Command::new(cmd).args(&args).current_dir(target).status();

    println!();

    match status {
        Ok(s) if s.success() => {
            let _ = execute!(
                handle,
                SetForegroundColor(Color::Green),
                Print("  🫠 Welcome back. We both knew you'd be back.\n"),
                Print("  Dependencies restored. Dignity... less so.\n"),
                ResetColor,
            );
        }
        _ => {
            let _ = execute!(
                handle,
                SetForegroundColor(Color::Red),
                Print("  💀 Reinstall failed. Even the comeback failed. Incredible.\n"),
                ResetColor,
            );
        }
    }

    println!();
}

fn detect_type(target: &Path) -> ProjectType {
    if target.join("package.json").exists() {
        ProjectType::Node
    } else if target.join("Cargo.toml").exists() {
        ProjectType::Rust
    } else if target.join("requirements.txt").exists() {
        ProjectType::Python
    } else if target.join("go.mod").exists() {
        ProjectType::Go
    } else {
        ProjectType::Unknown
    }
}
