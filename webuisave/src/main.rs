use std::process::Command;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
	#[arg(short, long)]
	commit: String,
}

fn main() {
	let args = Args::parse();
	run("echo", &format!("Commit Messge: {}", &args.commit));
	run("cargo", "build");
	run("cargo", "test");
	run_ma("git", &["add", "-A"]);
	run_ma("git", &["commit", "-m", &args.commit]);
	run_ma("git", &["push", "-u", "origin", "main"]);
	run("pwsh", "./IncrementVersion.ps1");
	run("cargo", "publish");
	run("echo", "Finished Successfully");
}

fn run(command:&str, commandarg: &str) {
	run_ma(command, &[commandarg]);
}

fn run_ma(command:&str, commandargs: &[&str]) {
	let output = Command::new(command).args(commandargs).output().expect("BAD");

	if !output.status.success() {
		let s = String::from_utf8_lossy(&output.stderr);
		panic!("Failed command {}:\n{}", command, s);
	}

	println!("{}", String::from_utf8_lossy(&output.stdout));
}
