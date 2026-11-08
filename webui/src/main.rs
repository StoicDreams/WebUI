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

#[cfg(test)]
mod tests {
    use super::*;

    fn version_in(text: &str) -> Option<&str> {
        let start = text.find("currentVersion")? + "currentVersion".len();
        let rest = text[start..].trim_start_matches([' ', '=']);
        let quote = rest.chars().next()?;
        rest[1..].split(quote).next()
    }

    #[test]
    fn the_starter_service_worker_matches_the_one_this_site_uses() {
        // The service worker's behavior is tested in the websites, which all share this file.
        let site = include_str!("../../webapp/root_files/service-worker.js");
        let without_version = |text: &str| -> String {
            text.replace('\r', "")
                .lines()
                .filter(|line| !line.starts_with("const currentVersion"))
                .collect::<Vec<_>>()
                .join("\n")
        };
        assert_eq!(without_version(SERVICEWORKER), without_version(site));
    }

    #[test]
    fn the_minified_starter_service_worker_has_the_same_version() {
        assert!(version_in(SERVICEWORKER).is_some());
        assert_eq!(version_in(SERVICEWORKERMIN), version_in(SERVICEWORKER));
    }

    #[test]
    fn the_starter_page_loads_the_minified_service_worker_that_ships_with_it() {
        let registers_min = [INDEX, WEBWORKERMAIN]
            .iter()
            .any(|text| text.contains("service-worker.min.js"));
        let registers_source = [INDEX, WEBWORKERMAIN]
            .iter()
            .any(|text| text.contains("service-worker.js"));
        assert!(
            registers_min || !registers_source,
            "the starter files register service-worker.js, which is not minified"
        );
    }

    #[test]
    fn the_starter_app_config_is_valid_json() {
        let config: serde_json::Value = serde_json::from_str(APPCONFIG).unwrap();
        assert!(config.is_object());
        let manifest: serde_json::Value = serde_json::from_str(MANIFEST).unwrap();
        assert!(manifest.is_object());
    }
}
