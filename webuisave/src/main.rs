use clap::Parser;
use std::fs;
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    commit: String,
    #[arg(long)]
    major: bool,
    #[arg(long)]
    minor: bool,
    #[arg(short, long)]
    publish: bool,
}

fn main() {
    let args = Args::parse();
    copy_static_files();

    if args.publish {
        let version_args = &mut Vec::new();
        version_args.push("./IncrementVersion.ps1");
        if args.major {
            version_args.push("-major");
        } else if args.minor {
            version_args.push("-minor");
        }
        run_ma("pwsh", &version_args);
    }
    run("cargo", "fmt");
    run("cargo", "update");
    run("cargo", "build");
    run("cargo", "test");
    run_ma("cargo", &["install", "--path", "webui"]);
    run_ma("git", &["add", "-A"]);
    run_ma("git", &["commit", "-m", &args.commit]);
    run_ma("git", &["push", "-u", "origin", "main"]);
    if args.publish {
        run_ma("cargo", &["publish", "-p", "webui"]);
    }
    run("pwsh", "./SyncVersionToLocalProjects.ps1");
    run("echo", "Finished Successfully");
}

/// Copy static files from webapp to webui
///
/// This is for WebUI dev only.
/// Active dev updates files in webapp.
/// When saving, we want to copy these files over to their counterpart in webui.
fn copy_static_files() {
    let web_app_css = "webapp/css/webui.css";
    let web_ui_css = "webui/src/static_files/css/webui.css";
    let web_app_js = "webapp/js/webui.js";
    let web_ui_js = "webui/src/static_files/js/webui.js";
    fs::copy(web_app_css, web_ui_css).unwrap();
    fs::copy(web_app_js, web_ui_js).unwrap();
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
