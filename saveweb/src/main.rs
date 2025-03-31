use clap::Parser;
use powershell_script::PsScriptBuilder;
use regex::Regex;
use std::{fs, path::Path};
use std::path::PathBuf;
use std::collections::HashMap;
use std::env;

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
    run_webui_if_webui_project();
    run_cargo_if_rust_project();
    build_sitemap();
    update_webdate_value();
    if let Some(commit) = args.commit {
        if args.noversion {
            // Skip incrementing versioning
        } else if args.major {
            increment_major_version("--major");
        } else if args.minor {
            increment_major_version("--minor");
        } else {
            increment_patch_version();
        }
        run_ma("git", &["add", "-A"], None);
        run_ma("git", &["commit", "-m", &format!("\"{}\"", &commit)], None);
        run_ma("git", &["push", "-u", "origin", "main"], None);
        if !args.noversion {
            let version = get_current_version();
            println!("Version:{:?}", version);
            if let Some(version) = version {
                run_ma("git", &["tag", "-a", &format!("v{}", version), "-m", &format!("\"Release v{}\"", version)], None);
                run_ma("git", &["push", "origin", "main", "--tags"], None);
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
        run_ma("cd", &[".."], None);
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

fn build_sitemap() {
    run_powershell_file_if_exists("./BuildSiteMap.ps1", "Sitemap Builder");
}

fn increment_patch_version() {
    run_powershell_file_if_exists("./PowerShell/IncrementVersion.ps1", "Increment Version");
    run_powershell_file_if_exists("./IncrementVersion.ps1", "Increment Version");
}

fn increment_major_version(flag:&str) {
    if let Some(file_path) = get_path("./PowerShell/IncrementVersion.ps1") {
        run_ma("pwsh", &[file_path.as_os_str().to_str().unwrap(), flag], None);
    } else if let Some(file_path) = get_path("./IncrementVersion.ps1") {
        run_ma("pwsh", &[file_path.as_os_str().to_str().unwrap(), flag], None);
    }
}

fn run_powershell_file_if_exists(file: &str, name: &str) {
    if let Some(file_path) = get_path(file) {
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
    run_ma(command, &[], directory);
}

fn run(command: &str, commandarg: &str, directory: Option<&str>) {
    run_ma(command, &[commandarg], directory);
}

fn run_ma(command: &str, commandargs: &[&str], directory: Option<&str>) {
    let ps = PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(false)
        .print_commands(true)
        .build();
    let mut script = format!("{} {}", command, commandargs.join(" "));
    if let Some(directory) = directory {
        script = format!(
            "cd {}
{}",
            directory, script
        );
    };
    println!("Running Command: {}", script);
    let output = ps.run(&script).unwrap();
    let result = output.stdout().unwrap_or_default();
    println!("Result: {}", result);
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
    match find_cargo_versions() {
        Ok(cargo_versions) => {
            for (_file, version) in cargo_versions {
                match version {
                    Some(v) => return Some(v),
                    None => return None
                }
            }
            find_readme_version()
        },
        Err(_) => find_readme_version()
    }
}

fn find_readme_version() -> Option<String> {
    if let Some(version) = find_readme_version_in_file("README.md") {
        return Some(version);
    }
    if let Some(version) = find_readme_version_in_file("Docs/README.md") {
        return Some(version);
    }
    None
}

fn find_readme_version_in_file(path: &str) -> Option<String> {
    let current_dir = env::current_dir().unwrap();
    let readme_path = current_dir.join(path);
    if readme_path.exists() {
        let readme_content = fs::read_to_string(readme_path).unwrap();
        for line in readme_content.lines() {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with("[Version: ") {
                let version = trimmed_line.split("[Version: ").nth(1).unwrap().split(']').nth(0).unwrap();
                return Some(version.to_string());
            }
        }
    }
    None
}

fn find_cargo_versions() -> std::io::Result<HashMap<PathBuf, Option<String>>> {
    let current_dir = env::current_dir()?;
    let mut versions = HashMap::new();
    visit_dirs(&current_dir, &mut versions)?;
    Ok(versions)
}

fn visit_dirs(dir: &Path, versions: &mut HashMap<PathBuf, Option<String>>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, versions)?;
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
