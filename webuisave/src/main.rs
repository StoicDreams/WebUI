use clap::Parser;
use std::fs;
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    commit: String,
}

fn main() {
    let args = Args::parse();
    copy_static_files();
    run("pwsh", "./IncrementVersion.ps1");
    run("cargo", "fmt");
    run("cargo", "update");
    run("cargo", "build");
    run("cargo", "test");
    run_ma("git", &["add", "-A"]);
    run_ma("git", &["commit", "-m", &args.commit]);
    run_ma("git", &["push", "-u", "origin", "main"]);
    run_ma("cargo", &["publish", "-p", "webui"]);
    run("echo", "Finished Successfully");
}

/// Copy static files from webapp to webui
///
/// Active dev updates files in webapp.
/// When saving, we want to copy these files over to their counterpart in webui.
fn copy_static_files() {
    fs::copy(
        "webapp/css/webui.css",
        "webui/src/static_files/css/webui.css",
    )
    .unwrap();
}

fn run(command: &str, commandarg: &str) {
    run_ma(command, &[commandarg]);
}

fn run_ma(command: &str, commandargs: &[&str]) {
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
