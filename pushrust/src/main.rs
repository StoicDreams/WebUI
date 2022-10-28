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
    run("cargo", "fmt");
    run("cargo", "update");
    run("cargo", "build");
    run("cargo", "test");
    run_ma("git", &["add", "-A"]);
    run_ma("git", &["commit", "-m", &args.commit]);
    run_ma("git", &["push", "-u", "origin", "main"]);
    run("echo", "Finished Successfully");
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
