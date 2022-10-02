mod build;

static INDEX: &str = include_str!("starter_files/index.html");
static MANIFEST: &str = include_str!("starter_files/app.webmanifest");
static LOGO: &str = include_str!("starter_files/Logo.svg");
static ROBOTS: &str = include_str!("starter_files/robots.txt");

static CSS: &str = include_str!("static_files/css/webui.css");

fn main() {
    build::add_file("css/webui.css", CSS);
    build::add_file_if_missing("index.html", INDEX);
    build::add_file_if_missing("app.webmanifest", MANIFEST);
    build::add_file_if_missing("Logo.svg", LOGO);
    build::add_file_if_missing("robots.txt", ROBOTS);
}
