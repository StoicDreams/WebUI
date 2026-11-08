use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::{fs, path::Path};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    commit: Option<String>,
    #[arg(long)]
    major: bool,
    #[arg(short = 'm', long)]
    minor: bool,
    #[arg(short = 'v', long)]
    noversion: bool,
}

fn main() {
    let args = Args::parse();
    check_solution_root();
    run(
        "echo",
        "echo \"Starting $(Split-Path -Path (Get-Location) -Leaf) ******\"",
        None,
    );
    build_setup();
    if args.commit.is_some() && !args.noversion {
        if args.major {
            increment_major_version("--major");
        } else if args.minor {
            increment_major_version("--minor");
        } else {
            increment_patch_version();
        }
    }
    run_webui_if_webui_project();
    run_cargo_if_rust_project();
    build_sitemap();
    update_webdate_value();
    if let Some(commit) = args.commit {
        run_pwsh("git", &["add", "-A"], None);
        run_pwsh("git", &["commit", "-m", &format!("\"{}\"", &commit)], None);
        run_pwsh("git", &["push", "-u", "origin", "main"], None);
        if !args.noversion {
            let version = get_current_version();
            println!("Version:{:?}", version);
            if let Some(version) = version {
                run_pwsh(
                    "git",
                    &[
                        "tag",
                        "-a",
                        &format!("v{}", version),
                        "-m",
                        &format!("\"Release v{}\"", version),
                    ],
                    None,
                );
                run_pwsh("git", &["push", "origin", "main", "--tags"], None);
            }
        }
    }
    run("echo", "Finished Successfully", None);
}

fn check_solution_root() {
    let nav_file = Path::new("./.git");
    if !nav_file.exists() {
        let nav_file = Path::new("../.git");
        if !nav_file.exists() {
            panic!("Must be in solution root folder to run this command.");
        }
        run_pwsh("cd", &[".."], None);
    }
}

fn run_webui_if_webui_project() {
    let nav_file = Path::new("./webapp");
    if !nav_file.exists() {
        return;
    }
    rc("webui", Some("webapp"));
}

fn run_cargo_if_rust_project() {
    let nav_file = Path::new("./Cargo.toml");
    if !nav_file.exists() {
        return;
    }
    delete_file_if_exists("./Cargo.lock");
    run("cargo", "fmt", None);
    run("cargo", "update", None);
    run("cargo", "build", None);
    run("cargo", "test", None);
}

fn build_setup() {
    run_powershell_file_if_exists("./BuildSetup.ps1", "Build Setup");
}

fn build_sitemap() {
    run_powershell_file_if_exists("./BuildSiteMap.ps1", "Sitemap Builder");
}

fn increment_patch_version() {
    run_powershell_file_if_exists("./IncrementVersion.ps1", "Increment Version");
}

fn increment_major_version(flag: &str) {
    if let Some(file_path) = get_path("./PowerShell/IncrementVersion.ps1") {
        run_pwsh(
            "pwsh",
            &[file_path.as_os_str().to_str().unwrap(), flag],
            None,
        );
    } else if let Some(file_path) = get_path("./IncrementVersion.ps1") {
        run_pwsh(
            "pwsh",
            &[file_path.as_os_str().to_str().unwrap(), flag],
            None,
        );
    }
}

fn run_powershell_file_if_exists(file: &str, name: &str) {
    if let Some(file_path) = get_path(file) {
        println!("Running {} ({})", name, file);
        run("pwsh", file_path.as_os_str().to_str().unwrap(), None);
    } else if let Some(file_path) = get_path(&file.replace("./", "./PowerShell/")) {
        println!("Running {} ({})", name, file);
        run("pwsh", file_path.as_os_str().to_str().unwrap(), None);
    }
}

fn get_path(file: &str) -> Option<&Path> {
    let file_path = Path::new(file);
    if file_path.exists() {
        Some(file_path)
    } else {
        None
    }
}

fn update_webdate_value() {
    update_webdate_value_for_file("./cdn/service-worker.js");
    update_webdate_value_for_file("./cdn/service-worker.min.js");
    update_webdate_value_for_file("./webapp/root_files/service-worker.js");
    update_webdate_value_for_file("./webapp/root_files/service-worker.min.js");
}

fn update_webdate_value_for_file(file: &str) {
    let webapp_file = Path::new(file);
    if !webapp_file.exists() {
        return;
    }
    let webapp_text = fs::read_to_string(webapp_file).unwrap();
    let timestamp = chrono::Utc::now().format("%y%m%d%H%M").to_string();
    let re = Regex::new(r"_ts_(\d+)").unwrap();
    let new_webapp_text = re
        .replace(&webapp_text, |_caps: &regex::Captures| {
            format!("_ts_{}", timestamp)
        })
        .to_string();
    fs::write(webapp_file, new_webapp_text).unwrap();
    println!("Updated {} with new timestamp: {}", file, timestamp);
}

fn rc(command: &str, directory: Option<&str>) {
    run_pwsh(command, &[], directory);
}

fn run(command: &str, commandarg: &str, directory: Option<&str>) {
    run_pwsh(command, &[commandarg], directory);
}

fn run_pwsh(command: &str, commandargs: &[&str], directory: Option<&str>) {
    let mut script = format!("{} {}", command, commandargs.join(" "));
    if let Some(directory) = directory {
        script = format!(
            "cd {}
{}",
            directory, script
        );
    };
    println!("Running Command: {}", script);
    let output = std::process::Command::new("pwsh")
        .arg("-Command")
        .arg(script)
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Output: {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", stderr);
    }
}

fn delete_file_if_exists(path: &str) {
    let fullpath = Path::new(path);
    if !fullpath.exists() {
        return;
    }
    match fs::remove_file(fullpath) {
        Ok(_) => {
            println!("Deleted file: {}", path);
        }
        Err(error) => {
            println!("Failed to delete file: {}", error);
        }
    };
}

fn get_current_version() -> Option<String> {
    let root = env::current_dir().ok()?;
    current_version_in(&root)
}

/// The version from the first Cargo.toml with a package version, in path order so the result is
/// the same every run, or else from the README.
fn current_version_in(root: &Path) -> Option<String> {
    let mut versions = HashMap::new();
    if visit_dirs(root, &mut versions).is_ok() {
        let mut paths: Vec<&PathBuf> = versions.keys().collect();
        paths.sort();
        if let Some(version) = paths.into_iter().find_map(|path| versions[path].clone()) {
            return Some(version);
        }
    }
    find_readme_version_in(root)
}

fn find_readme_version_in(root: &Path) -> Option<String> {
    ["README.md", "Docs/README.md"]
        .iter()
        .find_map(|path| find_readme_version_in_file(&root.join(path)))
}

fn find_readme_version_in_file(readme_path: &Path) -> Option<String> {
    let readme_content = fs::read_to_string(readme_path).ok()?;
    readme_content.lines().find_map(|line| {
        let rest = line.trim().strip_prefix("[Version: ")?;
        Some(rest.split(']').next().unwrap_or_default().to_string())
    })
}

fn visit_dirs(dir: &Path, versions: &mut HashMap<PathBuf, Option<String>>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let skip = path.file_name().is_some_and(|name| {
                    matches!(name.to_str(), Some("target" | "node_modules" | ".git"))
                });
                if !skip {
                    visit_dirs(&path, versions)?;
                }
            } else if path.file_name().map_or(false, |name| name == "Cargo.toml") {
                let version = extract_version(&path);
                versions.insert(path, version);
            }
        }
    }
    Ok(())
}

fn extract_version(file_path: &Path) -> Option<String> {
    if let Ok(contents) = fs::read_to_string(file_path) {
        let mut in_package_section = false;
        for line in contents.lines() {
            let trimmed_line = line.trim();
            if trimmed_line == "[package]" {
                in_package_section = true;
            } else if in_package_section {
                if trimmed_line.starts_with('[') {
                    break;
                }
                if let Some(stripped) = trimmed_line.strip_prefix("version") {
                    if let Some(version) = stripped.split('=').nth(1) {
                        let version = version.split('#').next().unwrap_or(version).trim();
                        let version = version.trim_matches(|c| c == '"' || c == '\'');
                        return Some(version.to_string());
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A new empty folder, removed when the test ends.
    struct TempDir(PathBuf);
    impl TempDir {
        fn new(name: &str) -> Self {
            let dir = env::temp_dir().join(format!("saveweb-{}-{}", name, std::process::id()));
            _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            TempDir(dir)
        }
        fn write(&self, path: &str, text: &str) {
            let path = self.0.join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, text).unwrap();
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn the_package_version_is_read_from_cargo_toml() {
        let dir = TempDir::new("package");
        dir.write(
            "Cargo.toml",
            "[package]\nname = \"x\"\nversion = \"1.2.3\" #syncwebui\n\n[dependencies]\nserde = { version = \"9.9.9\" }\n",
        );
        assert_eq!(
            extract_version(&dir.0.join("Cargo.toml")).as_deref(),
            Some("1.2.3")
        );
    }

    #[test]
    fn a_dependency_version_is_not_the_package_version() {
        let dir = TempDir::new("dependency");
        dir.write(
            "Cargo.toml",
            "[workspace]\nmembers = []\n\n[dependencies]\nversion = \"9.9.9\"\n",
        );
        assert_eq!(extract_version(&dir.0.join("Cargo.toml")), None);
    }

    #[test]
    fn a_workspace_root_without_a_version_does_not_hide_its_members_version() {
        let dir = TempDir::new("workspace");
        dir.write("Cargo.toml", "[workspace]\nmembers = [\"webapp\"]\n");
        dir.write(
            "webapp/Cargo.toml",
            "[package]\nname = \"webapp\"\nversion = \"0.4.2\"\n",
        );
        assert_eq!(current_version_in(&dir.0).as_deref(), Some("0.4.2"));
    }

    #[test]
    fn build_and_dependency_folders_are_not_searched() {
        let dir = TempDir::new("skip");
        dir.write(
            "target/package/Cargo.toml",
            "[package]\nversion = \"0.0.1\"\n",
        );
        dir.write(
            "node_modules/x/Cargo.toml",
            "[package]\nversion = \"0.0.2\"\n",
        );
        dir.write(
            "Docs/README.md",
            "# Site\n\n[Version: 2.0.0](https://example.com)\n",
        );
        assert_eq!(current_version_in(&dir.0).as_deref(), Some("2.0.0"));
    }

    #[test]
    fn the_readme_version_is_used_when_there_is_no_cargo_version() {
        let dir = TempDir::new("readme");
        dir.write(
            "README.md",
            "# CDN\n\n  [Version: 0.11.49](https://github.com/x)\n",
        );
        assert_eq!(current_version_in(&dir.0).as_deref(), Some("0.11.49"));
    }

    #[test]
    fn with_no_version_anywhere_there_is_none() {
        let dir = TempDir::new("none");
        dir.write("README.md", "# Nothing here\n");
        assert_eq!(current_version_in(&dir.0), None);
    }
}
