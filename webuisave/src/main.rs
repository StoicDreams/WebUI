use clap::Parser;
use std::path::Path;
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    commit: Option<String>,
    #[arg(long)]
    major: bool,
    #[arg(long)]
    minor: bool,
    #[arg(short, long)]
    publish: bool,
}

fn main() {
    let args = Args::parse();
    check_correct_folder();
    run("cargo", "test");
    run("cargo", "fmt");
    run("cargo", "update");
    run("cargo", "build");
    build_sitemap();
    if args.commit.is_some() {
        let version_args = &mut Vec::new();
        version_args.push("./IncrementVersion.ps1");
        if args.major {
            version_args.push("-major");
        } else if args.minor {
            version_args.push("-minor");
        }
        run_ma("pwsh", version_args);
    }
    run_ma("cargo", &["install", "--path", "webui"]);
    if let Some(commit) = args.commit {
        if args.publish {
            run_ma("git", &["add", "-A"]);
            run_ma("git", &["commit", "-m", &commit]);
            run_ma("git", &["push", "-u", "origin", "main"]);
            run_ma("cargo", &["publish", "-p", "webui_procs"]);
            run("cargo", "update");
            run("cargo", "build");
            run_ma("git", &["add", "-A"]);
            run_ma("git", &["commit", "-m", &commit]);
            run_ma("git", &["push", "-u", "origin", "main"]);
            run_ma("cargo", &["publish", "-p", "webui"]);
        } else {
            run("cargo", "update");
            run("cargo", "build");
            run_ma("git", &["add", "-A"]);
            run_ma("git", &["commit", "-m", &commit]);
            run_ma("git", &["push", "-u", "origin", "main"]);
        }
    }
    run("echo", "Finished Successfully");
    run(
        "echo",
        "Don't forget to run `cargo install --path webuisave`",
    );
}

fn check_correct_folder() {
    let nav_file = Path::new("./webui/src/static_files");
    if !nav_file.exists() {
        let nav_file = Path::new("../webui/src/static_files");
        if !nav_file.exists() {
            panic!("Must be in solution root folder to run this command.");
        }
        run_ma("cd", &[".."]);
    }
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
    if command == "echo" {
        println!("{}", commandarg);
        return;
    }
    run_ma(command, &[commandarg]);
}

fn run_ma(command: &str, commandargs: &[&str]) {
    println!("Running Command: {} {:?}", command, commandargs);
    let output = Command::new(command)
        .args(commandargs)
        .output()
        .expect("Command failed");

    if !output.status.success() {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("Failed command {}:\n{}", command, s);
        // If a commit failed we'll assume it's because nothing needed committing
        if command == "git" && commandargs.iter().any(|val| *val == "commit") {
            return;
        }

        panic!("Failed command {}:\n{}", command, s);
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
