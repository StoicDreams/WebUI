use clap::Parser;
use powershell_script::PsScriptBuilder;
use std::{fs, path::Path};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    commit: Option<String>,
}

fn main() {
    let args = Args::parse();
    check_correct_folder();
    run(
        "echo",
        "echo \"Starting $(Split-Path -Path (Get-Location) -Leaf) ******\"",
        None,
    );
    rc("webui", Some("webapp"));
    delete_file_if_exists("./Cargo.lock");
    run("cargo", "fmt", None);
    run("cargo", "update", None);
    run("cargo", "build", None);
    run("cargo", "test", None);
    build_sitemap();
    if let Some(commit) = args.commit {
        run_ma("git", &["add", "-A"], None);
        run_ma("git", &["commit", "-m", &format!("\"{}\"", &commit)], None);
        run_ma("git", &["push", "-u", "origin", "main"], None);
    }
    run("echo", "Finished Successfully", None);
}

fn check_correct_folder() {
    let nav_file = Path::new("./webapp");
    if !nav_file.exists() {
        let nav_file = Path::new("../webapp");
        if !nav_file.exists() {
            panic!("Must be in solution root folder to run this command.");
        }
        run_ma("cd", &[".."], None);
    }
}

fn build_sitemap() {
    let sitemap_file = Path::new("./PowerShell/BuildSiteMap.ps1");
    if !sitemap_file.exists() {
        println!("Missing Sitemap Builder - expected file PowerShell/BuildSiteMap.ps1");
        return;
    }
    println!("Running Sitemap Builder");
    run("pwsh", sitemap_file.as_os_str().to_str().unwrap(), None);
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
