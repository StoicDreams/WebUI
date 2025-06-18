#![allow(unused)] // TODO: Remove me when needing to check for dead code / unused methods/variables.

mod starter_files;

static APPCONFIG: &str = include_str!("starter_files/appConfig.json");
static INDEX: &str = include_str!("starter_files/index.html");
static MANIFEST: &str = include_str!("starter_files/app.webmanifest");
static LOGO: &str = include_str!("starter_files/Logo.svg");
static ROBOTS: &str = include_str!("starter_files/robots.txt");
static SERVICEWORKER: &str = include_str!("starter_files/service-worker.js");
static SERVICEWORKERMIN: &str = include_str!("starter_files/service-worker.min.js");
static WEBWORKERMAIN: &str = include_str!("starter_files/main.js");
static WEBWORKERLIB: &str = include_str!("starter_files/worker.js");

fn main() {
    starter_files::add_file_if_missing("root_files/appConfig.json", APPCONFIG);
    starter_files::add_file_if_missing("index.html", INDEX);
    starter_files::add_file_if_missing("root_files/app.webmanifest", MANIFEST);
    starter_files::add_file_if_missing("root_files/Logo.svg", LOGO);
    starter_files::add_file_if_missing("root_files/robots.txt", ROBOTS);
    starter_files::add_file_if_missing("root_files/service-worker.js", SERVICEWORKER);
    starter_files::add_file_if_missing("root_files/service-worker.min.js", SERVICEWORKERMIN);
    starter_files::add_file_if_missing("src-js/main.js", WEBWORKERMAIN);
    starter_files::add_file_if_missing("src-js/worker.js", WEBWORKERLIB);
}
