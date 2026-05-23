use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct ProjectStats {
    pub project_name: String,
    pub project_type: ProjectType,
    pub total_files: usize,
    pub git_commits: Option<usize>,
    pub git_age_days: Option<u64>,
    pub dependency_count: usize,
    pub largest_file: Option<(String, usize)>,
    pub utils_file_count: usize,
    pub todo_count: usize,
    pub bloat_dirs: Vec<BloatDir>,
    pub total_bloat_bytes: u64,
}

#[derive(Debug)]
pub struct BloatDir {
    pub path: PathBuf,
    pub label: String,
    pub size_bytes: u64,
    pub destruction_verb: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectType {
    Node,
    Rust,
    Python,
    Go,
    Unknown,
}

impl std::fmt::Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectType::Node => write!(f, "Node.js"),
            ProjectType::Rust => write!(f, "Rust"),
            ProjectType::Python => write!(f, "Python"),
            ProjectType::Go => write!(f, "Go"),
            ProjectType::Unknown => write!(f, "Unknown"),
        }
    }
}

pub fn analyze(target: &Path) -> ProjectStats {
    let project_name = target
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unnamed-project".to_string());

    let project_type = detect_project_type(target);
    let git_commits = get_git_commits(target);
    let git_age_days = get_git_age_days(target);
    let dependency_count = count_dependencies(target, &project_type);

    let mut total_files = 0usize;
    let mut largest_file: Option<(String, usize)> = None;
    let mut utils_file_count = 0usize;
    let mut todo_count = 0usize;

    let skip_dirs = [
        "node_modules",
        "target",
        "dist",
        ".next",
        ".git",
        "__pycache__",
        ".turbo",
        "build",
        ".cache",
        "vendor",
    ];

    for entry in WalkDir::new(target)
        .max_depth(6)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !skip_dirs.contains(&name.as_ref())
        })
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            total_files += 1;
            let fname = entry.file_name().to_string_lossy().to_lowercase();

            if fname.contains("util") {
                utils_file_count += 1;
            }

            // Count lines and TODOs for source files
            let ext = entry
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            let is_source = matches!(
                ext,
                "js" | "ts"
                    | "tsx"
                    | "jsx"
                    | "rs"
                    | "py"
                    | "go"
                    | "java"
                    | "rb"
                    | "css"
                    | "html"
                    | "vue"
                    | "svelte"
                    | "c"
                    | "cpp"
                    | "h"
            );

            if is_source {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    let line_count = content.lines().count();

                    // Track largest file
                    if let Some((_, current_max)) = &largest_file {
                        if line_count > *current_max {
                            largest_file = Some((fname.clone(), line_count));
                        }
                    } else if line_count > 50 {
                        largest_file = Some((fname.clone(), line_count));
                    }

                    // Count TODOs
                    for line in content.lines() {
                        let upper = line.to_uppercase();
                        if upper.contains("TODO")
                            || upper.contains("FIXME")
                            || upper.contains("HACK")
                            || upper.contains("XXX")
                        {
                            todo_count += 1;
                        }
                    }
                }
            }
        }
    }

    let bloat_dirs = detect_bloat_dirs(target);
    let total_bloat_bytes = bloat_dirs.iter().map(|d| d.size_bytes).sum();

    ProjectStats {
        project_name,
        project_type,
        total_files,
        git_commits,
        git_age_days,
        dependency_count,
        largest_file,
        utils_file_count,
        todo_count,
        bloat_dirs,
        total_bloat_bytes,
    }
}

fn detect_project_type(target: &Path) -> ProjectType {
    if target.join("package.json").exists() {
        ProjectType::Node
    } else if target.join("Cargo.toml").exists() {
        ProjectType::Rust
    } else if target.join("requirements.txt").exists()
        || target.join("pyproject.toml").exists()
        || target.join("setup.py").exists()
    {
        ProjectType::Python
    } else if target.join("go.mod").exists() {
        ProjectType::Go
    } else {
        ProjectType::Unknown
    }
}

fn get_git_commits(target: &Path) -> Option<usize> {
    Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .current_dir(target)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout)
                    .ok()
                    .and_then(|s| s.trim().parse().ok())
            } else {
                None
            }
        })
}

fn get_git_age_days(target: &Path) -> Option<u64> {
    Command::new("git")
        .args(["log", "--reverse", "--format=%ct", "-1"])
        .current_dir(target)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                let ts_str = String::from_utf8(o.stdout).ok()?;
                let ts: i64 = ts_str.trim().parse().ok()?;
                let now = chrono::Utc::now().timestamp();
                Some(((now - ts) / 86400) as u64)
            } else {
                None
            }
        })
}

fn count_dependencies(target: &Path, project_type: &ProjectType) -> usize {
    match project_type {
        ProjectType::Node => {
            if let Ok(content) = std::fs::read_to_string(target.join("package.json")) {
                // Simple counting: count occurrences of version-like patterns in deps
                let deps = content.matches("\"dependencies\"").count();
                let dev_deps = content.matches("\"devDependencies\"").count();
                if deps > 0 || dev_deps > 0 {
                    // Count lines between braces for each section
                    let mut count = 0;
                    let mut in_deps = false;
                    let mut brace_depth = 0;
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if trimmed.contains("\"dependencies\"")
                            || trimmed.contains("\"devDependencies\"")
                            || trimmed.contains("\"peerDependencies\"")
                        {
                            in_deps = true;
                            brace_depth = 0;
                            continue;
                        }
                        if in_deps {
                            if trimmed.contains('{') {
                                brace_depth += 1;
                            }
                            if trimmed.contains('}') {
                                brace_depth -= 1;
                                if brace_depth <= 0 {
                                    in_deps = false;
                                }
                                continue;
                            }
                            if trimmed.contains(':') && brace_depth > 0 {
                                count += 1;
                            }
                        }
                    }
                    return count;
                }
            }
            0
        }
        ProjectType::Rust => {
            if let Ok(content) = std::fs::read_to_string(target.join("Cargo.toml")) {
                let mut count = 0;
                let mut in_deps = false;
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed == "[dependencies]"
                        || trimmed == "[dev-dependencies]"
                        || trimmed == "[build-dependencies]"
                    {
                        in_deps = true;
                        continue;
                    }
                    if trimmed.starts_with('[') {
                        in_deps = false;
                        continue;
                    }
                    if in_deps
                        && trimmed.contains('=')
                        && !trimmed.is_empty()
                        && !trimmed.starts_with('#')
                    {
                        count += 1;
                    }
                }
                return count;
            }
            0
        }
        ProjectType::Python => {
            if let Ok(content) = std::fs::read_to_string(target.join("requirements.txt")) {
                return content
                    .lines()
                    .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
                    .count();
            }
            0
        }
        ProjectType::Go => {
            if let Ok(content) = std::fs::read_to_string(target.join("go.mod")) {
                return content
                    .lines()
                    .filter(|l| {
                        l.trim().starts_with("require") || (l.contains('/') && l.contains(' '))
                    })
                    .count();
            }
            0
        }
        ProjectType::Unknown => 0,
    }
}

fn detect_bloat_dirs(target: &Path) -> Vec<BloatDir> {
    let candidates = vec![
        ("node_modules", "Incinerating"),
        (".next", "Obliterating"),
        ("dist", "Vaporizing"),
        ("build", "Demolishing"),
        (".turbo", "Annihilating"),
        ("target", "Detonating"),
        ("__pycache__", "Exterminating"),
        (".cache", "Purging"),
        (".parcel-cache", "Liquidating"),
        ("coverage", "Shredding"),
        (".nyc_output", "Disintegrating"),
    ];

    let mut bloat_dirs = Vec::new();

    for (dir_name, verb) in candidates {
        let path = target.join(dir_name);
        if path.is_dir() {
            let size = dir_size(&path);
            bloat_dirs.push(BloatDir {
                path,
                label: dir_name.to_string(),
                size_bytes: size,
                destruction_verb: verb.to_string(),
            });
        }
    }

    bloat_dirs
}

fn dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.metadata().map(|m| m.len()).unwrap_or(0))
        .sum()
}
