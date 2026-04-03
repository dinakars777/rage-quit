# rage-quit 🎤💥

The most dramatic way to leave a project. Ever.

Because `exit` is too peaceful.

![Demo](demo.gif)

## Install

```bash
cargo install rage-quit-cli
```

## Usage

```bash
# Full cinematic experience (safe — doesn't delete anything)
rage-quit

# Actually delete node_modules, dist, .next, target, etc.
rage-quit --nuke

# Changed your mind? Reinstall everything
rage-quit --comeback

# Just show the resignation letter
rage-quit --letter-only

# Skip animations
rage-quit --silent --nuke
```

## What Happens

1. 🔥 Your terminal erupts in flames
2. 💀 Fake error messages cascade across your screen
3. 📊 Bloat directories get (optionally) incinerated with dramatic progress bars
4. 📝 A personalized resignation letter is generated from your project stats
5. 🎤 Mic drop

## The Resignation Letter

`rage-quit` analyzes your actual project to generate personalized grievances:

- How many commits you've suffered through
- The size of your `node_modules` black hole
- How many files are named "utils"
- That one 2000-line file everyone's afraid to touch
- Your unfulfilled TODO count

## Options

| Flag | Description |
|------|-------------|
| `--nuke` | Actually delete bloat directories |
| `--comeback` | Undo the rage quit (reinstall deps) |
| `--silent` | Skip animations |
| `--letter-only` | Only show the resignation letter |
| `--speed` | Animation speed: `slow`, `normal`, `fast` |
| `--target <PATH>` | Target a specific directory |

## Supported Project Types

- 📦 Node.js (npm/yarn/pnpm)
- 🦀 Rust (Cargo)
- 🐍 Python (pip/poetry)
- 🐹 Go (go mod)

## Contributing

Contributions welcome! Especially:
- [ ] More dramatic animations
- [ ] Sound effects mode (terminal bell abuse)
- [ ] Team rage-quit (notify Slack before quitting)
- [ ] Rage quit leaderboard

## License

MIT
