use std::env;
use std::fs;
use std::path::Path;

pub fn add_file(dest_path: &str, contents: &str) {
    let out_dir = env::current_dir().unwrap().display().to_string();
    let dest_path = Path::new(&out_dir).join(dest_path);
    fs::create_dir_all(Path::parent(&dest_path).unwrap()).unwrap();
    fs::write(&dest_path, &contents).unwrap();
}

pub fn add_file_if_missing(dest_path: &str, contents: &str) {
    let out_dir = env::current_dir().unwrap().display().to_string();
    let dest_path = Path::new(&out_dir).join(dest_path);
    if dest_path.exists() {
        return;
    }
    fs::create_dir_all(Path::parent(&dest_path).unwrap()).unwrap();
    fs::write(&dest_path, &contents).unwrap();
}
