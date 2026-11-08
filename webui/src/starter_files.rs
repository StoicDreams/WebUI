use std::env;
use std::fs;
use std::path::Path;

pub(crate) fn add_file(dest_path: &str, contents: &str) {
    add_file_in(&env::current_dir().unwrap(), dest_path, contents);
}

pub(crate) fn add_file_if_missing(dest_path: &str, contents: &str) {
    add_file_if_missing_in(&env::current_dir().unwrap(), dest_path, contents);
}

/// Write a file under root, creating its folders, and replacing any file already there.
fn add_file_in(root: &Path, dest_path: &str, contents: &str) {
    let dest_path = root.join(dest_path);
    println!(
        "WebUI Copying Static File {}",
        Path::file_name(&dest_path).unwrap().to_str().unwrap()
    );
    fs::create_dir_all(Path::parent(&dest_path).unwrap()).unwrap();
    fs::write(&dest_path, contents).unwrap();
}

/// Write a starter file under root, unless the site already has that file.
fn add_file_if_missing_in(root: &Path, dest_path: &str, contents: &str) {
    let dest_path = root.join(dest_path);
    if dest_path.exists() {
        println!(
            "WebUI Skipping Starter File {} - already exists.",
            Path::file_name(&dest_path).unwrap().to_str().unwrap()
        );
        return;
    }
    println!(
        "WebUI Copying Starter File {}",
        Path::file_name(&dest_path).unwrap().to_str().unwrap()
    );
    fs::create_dir_all(Path::parent(&dest_path).unwrap()).unwrap();
    fs::write(&dest_path, contents).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// A new empty folder, removed when the test ends.
    struct TempDir(PathBuf);
    impl TempDir {
        fn new(name: &str) -> Self {
            let dir =
                env::temp_dir().join(format!("webui-starter-{}-{}", name, std::process::id()));
            _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            TempDir(dir)
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn a_starter_file_is_written_with_its_folders() {
        let dir = TempDir::new("new");
        add_file_if_missing_in(&dir.0, "root_files/robots.txt", "allow");
        assert_eq!(
            fs::read_to_string(dir.0.join("root_files/robots.txt")).unwrap(),
            "allow"
        );
    }

    #[test]
    fn a_starter_file_never_replaces_the_sites_own_file() {
        let dir = TempDir::new("existing");
        add_file_in(&dir.0, "index.html", "the site's own page");
        add_file_if_missing_in(&dir.0, "index.html", "starter page");
        assert_eq!(
            fs::read_to_string(dir.0.join("index.html")).unwrap(),
            "the site's own page"
        );
    }

    #[test]
    fn a_static_file_replaces_what_is_there() {
        let dir = TempDir::new("static");
        add_file_in(&dir.0, "a/b.txt", "old");
        add_file_in(&dir.0, "a/b.txt", "new");
        assert_eq!(fs::read_to_string(dir.0.join("a/b.txt")).unwrap(), "new");
    }
}
