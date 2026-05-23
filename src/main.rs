mod analyzer;
mod animation;
mod comeback;
mod phases;
mod sound;

use clap::Parser;
use crossterm::terminal;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "rage-quit",
    version,
    about = "🎤💥 The most dramatic way to leave a project. Ever.",
    long_about = "Because `exit` is too peaceful.\n\n\
        Run rage-quit in any project directory for a cinematic rage quit experience.\n\
        Use --nuke to actually delete bloat directories (node_modules, dist, target, etc.).\n\
        Use --comeback when you inevitably crawl back."
)]
struct Cli {
    /// Actually delete bloat directories (node_modules, dist, .next, target, etc.)
    #[arg(long)]
    nuke: bool,

    /// Undo the rage quit (reinstall dependencies)
    #[arg(long)]
    comeback: bool,

    /// Skip animations, just show the resignation letter
    #[arg(long)]
    silent: bool,

    /// Only generate and display the resignation letter
    #[arg(long)]
    letter_only: bool,

    /// Enable sound effects (terminal bells)
    #[arg(long)]
    sound: bool,

    /// Use only terminal bells (no audio files)
    #[arg(long)]
    bell_only: bool,

    /// Animation speed: slow, normal, fast
    #[arg(long, value_enum, default_value_t = animation::AnimationSpeed::Normal)]
    speed: animation::AnimationSpeed,

    /// Target a specific directory
    #[arg(long)]
    target: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    animation::set_speed(cli.speed);

    let target = cli
        .target
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    // Create sound player
    let sound_player = sound::create_player(cli.silent, cli.sound, cli.bell_only);

    // Handle comeback mode
    if cli.comeback {
        comeback::run(&target);
        return;
    }

    // Analyze the project
    let stats = analyzer::analyze(&target);

    // Get terminal dimensions
    let (width, height) = terminal::size().unwrap_or((100, 24));

    // Letter-only mode
    if cli.letter_only {
        phases::letter::run(&stats, width);
        return;
    }

    // Silent mode
    if cli.silent {
        if cli.nuke {
            phases::cleanup::run(&stats, true, &sound_player);
        }
        phases::letter::run(&stats, width);
        return;
    }

    // ===== FULL CINEMATIC EXPERIENCE =====

    // Phase 1: The Buildup
    phases::buildup::run(width, height, &sound_player);

    // Phase 2: The Explosion
    phases::explosion::run(width, height, &sound_player);

    // Phase 3: The Cleanup
    phases::cleanup::run(&stats, cli.nuke, &sound_player);

    // Phase 4: The Letter
    phases::letter::run(&stats, width);

    // Phase 5: The Exit
    phases::exit::run(width, &sound_player);
}
