use clap::Parser;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    commit: String,
}

fn main() {
    let args = Args::parse();
    run("cargo", "fmt");
    run("cargo", "update");
    run("cargo", "build");
    run("cargo", "test");
    build_sitemap();
    run_ma("git", &["add", "-A"]);
    run_ma("git", &["commit", "-m", &args.commit]);
    run_ma("git", &["push", "-u", "origin", "main"]);
    run("echo", "Finished Successfully");
}

fn build_sitemap() {
    let sitemap_file = Path::new("./PowerShell/BuildSiteMap.ps1");
    if !sitemap_file.exists() {
        println!("Missing Sitemap Builder - expected file PowerShell/BuildSiteMap.ps1");
        return;
    }
    let nav_file = Path::new("./webapp/src/nav_menu.rs");
    if !nav_file.exists() {
        println!("Missing Nav Menu File - expected file webapp/src/nav_menu.rs");
        return;
    }
    println!("Running Sitemap Builder");
    run("pwsh", sitemap_file.as_os_str().to_str().unwrap());
}

fn run(command: &str, commandarg: &str) {
    run_ma(command, &[commandarg]);
}

fn run_ma(command: &str, commandargs: &[&str]) {
    println!("Running Command: {} {:?}", command, commandargs);
    let output = Command::new(command)
        .args(commandargs)
        .output()
        .expect("BAD");

    if !output.status.success() {
        let s = String::from_utf8_lossy(&output.stderr);
        panic!("Failed command {}:\n{}", command, s);
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
