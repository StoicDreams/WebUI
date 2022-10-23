mod main_mods;

use main_mods::starter_files;

static INDEX: &str = include_str!("starter_files/index.html");
static MANIFEST: &str = include_str!("starter_files/app.webmanifest");
static LOGO: &str = include_str!("starter_files/Logo.svg");
static ROBOTS: &str = include_str!("starter_files/robots.txt");
static SERVICEWORKER: &str = include_str!("starter_files/service-worker.js");

static WEBUIINTEROPJS: &str = include_str!("static_files/js/webui_interop.js");
static WEBUIJS: &str = include_str!("static_files/js/webui.js");
static CSS: &str = include_str!("static_files/css/webui.css");

fn main() {
    starter_files::add_file("js/webui_interop.js", WEBUIINTEROPJS);
    starter_files::add_file("js/webui.js", WEBUIJS);
    starter_files::add_file("css/webui.css", CSS);
    starter_files::add_file_if_missing("index.html", INDEX);
    starter_files::add_file_if_missing("app.webmanifest", MANIFEST);
    starter_files::add_file_if_missing("Logo.svg", LOGO);
    starter_files::add_file_if_missing("robots.txt", ROBOTS);
    starter_files::add_file_if_missing("service-worker.js", SERVICEWORKER);
}
