use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

mod main_wasm;
mod preload;
mod prerender;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Write a prerendered HTML file for every page into webapp/pages, to commit with the site.
    Prerender {
        /// The site's webapp folder.
        #[arg(default_value = ".")]
        webapp: String,
    },
}

fn main() {
    let args = Args::parse();
    if let Some(Command::Prerender { webapp }) = args.command {
        match prerender::generate(Path::new(&webapp)) {
            Ok(count) => println!("Prerender: wrote {count} pages to {webapp}/{}", prerender::PAGES_DIR),
            Err(err) => {
                eprintln!("Prerender failed: {err}");
                std::process::exit(1);
            }
        }
        return;
    }
    println!("Starting WebApp Post Build!");
    // Trunk tells hooks where the staged build is. Older Trunk versions do not, so fall back to its default.
    let stage = std::env::var("TRUNK_STAGING_DIR").unwrap_or_else(|_| "dist/.stage".to_string());
    let stage = Path::new(&stage);
    // Trunk copies root_files/ into the build as a folder. Its files belong at the site root, and leaving the folder
    // served every one of them a second time under /root_files/.
    let root_files = stage.join("root_files");
    if root_files.is_dir() {
        copy_dir(&root_files, stage);
        let _ = fs::remove_dir_all(&root_files);
    }
    // Before the pages go in, so it finds Trunk's start script in the index.html Trunk wrote.
    main_wasm::run(stage);
    let pages = stage.join(prerender::PAGES_DIR);
    if pages.is_dir() {
        // The site commits its prerendered pages (webapp_post_build prerender). Release builds serve them from the
        // site root, replacing Trunk's index.html. Development builds keep Trunk's, so changes to index.html show.
        let release = std::env::var("TRUNK_PROFILE").map_or(true, |profile| profile == "release");
        if release {
            copy_dir(&pages, stage);
            println!("Prerendered pages copied from {}", prerender::PAGES_DIR);
        }
        let _ = fs::remove_dir_all(&pages);
    } else {
        prerender::run(stage);
    }
    println!("Finished Web UI Post Build Successfully!");
}

fn copy_dir(from: &Path, to: &Path) {
    let Ok(entries) = fs::read_dir(from) else {
        return;
    };
    for entry in entries.flatten() {
        let source = entry.path();
        let target = to.join(entry.file_name());
        if source.is_dir() {
            fs::create_dir_all(&target).unwrap();
            copy_dir(&source, &target);
        } else {
            println!("File:{} from {}", target.display(), source.display());
            fs::copy(&source, &target).unwrap();
        }
    }
}

// fn rc(command: &str, directory: Option<&str>) {
//     run_ma(command, &[], directory);
// }

// fn run(command: &str, commandarg: &str, directory: Option<&str>) {
//     run_ma(command, &[commandarg], directory);
// }

// fn run_ma(command: &str, commandargs: &[&str], directory: Option<&str>) {
//     println!("Running Command: {} {:?}", command, commandargs);
//     let mut com = Command::new(command);
//     let com = com.args(commandargs);
//     match directory {
//         Some(directory) => {
//             com.current_dir(directory);
//         }
//         None => (),
//     };
//     let output = com.output().expect("BAD");

//     if !output.status.success() {
//         let s = String::from_utf8_lossy(&output.stderr);
//         panic!("Failed command {}:\n{}", command, s);
//     }

//     println!("{}", String::from_utf8_lossy(&output.stdout));
// }
