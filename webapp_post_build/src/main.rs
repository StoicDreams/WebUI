use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    todo: Option<String>,
}

fn main() {
    let _args = Args::parse();
    println!("Starting WebApp Post Build!");
    copy_root_files_to_dist();
    println!("Finished Web UI Post Build Successfully!");
}

fn copy_root_files_to_dist() {
    let path = Path::new(DIR_ROOT_FILES);
    if !path.exists() {
        return;
    }
    copy_files_to_dist(DIR_ROOT_FILES);
}

const DIR_ROOT_FILES: &str = "dist/.stage/root_files";

fn copy_files_to_dist(directory: &str) {
    println!("Directory:{}", directory);
    let target = directory.replace(DIR_ROOT_FILES, "dist/.stage");
    fs::create_dir_all(target).unwrap();
    let dir = fs::read_dir(directory);
    let files = match dir {
        Ok(dir) => dir
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap_or_default(),
        Err(_) => Vec::new(),
    };
    for buf in files {
        if buf.is_dir() {
            copy_files_to_dist(buf.as_os_str().to_str().unwrap());
        } else {
            let source = buf.as_os_str().to_str().unwrap();
            let target = source.replace(DIR_ROOT_FILES, "dist/.stage");
            println!("File:{} from {}", target, source);
            fs::copy(source, target).unwrap();
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
