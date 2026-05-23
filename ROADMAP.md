# rage-quit Roadmap

This roadmap is based on the current repository state after the initial health pass and merged PRs #1 through #5.

## Recently Addressed

- Strict local quality gate now passes with `cargo fmt --check`, `cargo test --locked`, and `cargo clippy --locked --all-targets --all-features -- -D warnings`.
- `Cargo.lock` is tracked for reproducible binary CLI builds.
- Node dependency counting now covers normal `dependencies`, `devDependencies`, and `peerDependencies` sections.
- Terminal truncation avoids narrow-width underflow and UTF-8 byte slicing.
- `--speed` is now a validated option and scales animation timing.

## Phase 1: Project Health And Release Safety

- Add GitHub Actions CI for `cargo fmt --check`, `cargo test --locked`, `cargo clippy --locked --all-targets --all-features -- -D warnings`, and a `cargo run --locked -- --help` smoke test.
- Add a release checklist covering version bump, changelog update, `cargo package`, install smoke test, demo refresh, and crates.io publish.
- Add PR and issue templates so bug reports capture terminal, shell, OS, command flags, and target project type.
- Keep README, docs site, changelog, and CLI help synchronized during every release.

## Phase 2: Analyzer Accuracy

- Replace the remaining ad hoc manifest scanners with focused parsers or well-tested helpers for `package.json`, `Cargo.toml`, `pyproject.toml`, `requirements.txt`, and `go.mod`.
- Add analyzer fixture tests for Node, Rust, Python, Go, mixed-language repos, malformed manifests, and empty projects.
- Improve dependency counts by recognizing lockfiles and package-manager variants such as npm, yarn, pnpm, Poetry, and Cargo workspaces.
- Improve bloat detection beyond top-level directories by supporting configurable max depth, excludes, and nested build/cache directories.

## Phase 3: Destructive-Mode Safety

- Split cleanup planning from deletion so `--nuke` can show an auditable deletion plan before any filesystem mutation.
- Add tests around cancellation, confirmation input, missing permissions, symlinks, and partial deletion failures.
- Report deletion failures instead of silently ignoring `remove_dir_all` errors.
- Consider an explicit `--yes` flag only after deletion planning and tests are in place.

## Phase 4: Terminal Experience

- Make `--silent` truly non-animated by bypassing progress delays and decorative output where possible.
- Add terminal capability checks for color, Unicode glyph support, and very small terminal sizes.
- Add optional theme/preset support for alternate copy and animation styles.
- Add a shareable `--output letter.md` mode for saving the generated resignation letter.

## Phase 5: Sound And Media

- Decide whether full audio should remain a roadmap item or become an optional feature with a real audio backend.
- If full audio ships, add an explicit Cargo feature, optional dependency, assets policy, and fallback behavior.
- Add automated demo generation so `demo.gif`, `docs/demo.cast`, README, and the docs site stay in sync.

## Phase 6: Website And Distribution

- Add a docs deployment workflow for the static site under `docs/`.
- Add install verification for `cargo install --path .` and, before release, `cargo install rage-quit-cli`.
- Add badges for crates.io version, CI status, license, and supported Rust version once CI exists.
- Document supported platforms and known shell/terminal limitations.
