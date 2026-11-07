//! Add preload hints for the components a prerendered page uses, so the browser fetches them alongside the
//! framework instead of one level at a time.
//!
//! Without hints, a component's script is only requested once the loader finds its tag, and a component that
//! uses other components only reveals them once its own script has run. Each level costs a network round trip.
//! The site's own components (`app-*`) are in the build, so their dependencies are followed all the way down.
//! Framework components (`webui-*`) live on the CDN, so only the ones used directly are known.
//!
//! The hints must match how the loader requests each script, or the browser downloads it twice:
//! `https://cdn.myfi.ws/webui/{name}.min.js` and `/wc/{name}.min.js`, both with `crossorigin="anonymous"`.

use regex::Regex;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

/// Where the loader loads framework components from.
pub const CDN_COMPONENTS: &str = "https://cdn.myfi.ws/webui/";
/// The markdown parser, which the loader imports on every page.
pub const MARKDOWN_PARSER: &str = "https://cdn.myfi.ws/js/mdparse.min.js";

/// Code examples, comments, scripts, styles, and templates, whose tags the loader never loads.
static RX_NOT_LOADED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?is)<!--.*?-->|```.*?```|`[^`\n]*`|<(script|style|template)\b[^>]*>.*?</(?:script|style|template)>|<webui-code\b[^>]*>.*?</webui-code>",
    )
    .unwrap()
});
static RX_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)<((?:webui|app)-[a-z0-9-]+)[\s/>]").unwrap());
static RX_PRELOAD_ATTR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)\spreload\s*=\s*(?:"([^"]*)"|'([^']*)')"#).unwrap());
// In component source: a preload option, and components the code creates or waits for.
static RX_PRELOAD_OPTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\bpreload\s*:\s*(?:'([^']*)'|"([^"]*)"|`([^`]*)`)"#).unwrap());
static RX_CODE_USE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:\bcreate|\bwhenDefined)\(\s*['"`]((?:webui|app)-[a-z0-9-]+)['"`]"#).unwrap()
});
static RX_TEMPLATE_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<((?:webui|app)-[a-z0-9-]+)[\s/>]").unwrap());

/// Turn a preload list, such as `"flex dropdown app:tool-output"`, into component names.
fn preload_names(list: &str) -> impl Iterator<Item = String> + '_ {
    list.split(|c: char| c == ',' || c == ';' || c.is_whitespace())
        .filter(|name| !name.is_empty())
        .map(|name| match name.strip_prefix("app:") {
            Some(app) => format!("app-{}", app.to_lowercase()),
            None => format!("webui-{}", name.to_lowercase()),
        })
}

/// Components used by page markup: their tags, and any preload attributes.
pub fn components_in_markup(text: &str) -> BTreeSet<String> {
    let text = RX_NOT_LOADED.replace_all(text, "");
    let mut names: BTreeSet<String> = RX_TAG
        .captures_iter(&text)
        .map(|caps| caps[1].to_lowercase())
        .collect();
    for caps in RX_PRELOAD_ATTR.captures_iter(&text) {
        let list = caps.get(1).or(caps.get(2)).map_or("", |m| m.as_str());
        names.extend(preload_names(list));
    }
    names
}

/// Components a component's source uses: its preload option, ones it creates or waits for, and tags in its
/// templates. Data keys such as `app-nav-routes` are not components, so only these places are read.
pub fn components_in_source(source: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for caps in RX_PRELOAD_OPTION.captures_iter(source) {
        let list = caps
            .get(1)
            .or(caps.get(2))
            .or(caps.get(3))
            .map_or("", |m| m.as_str());
        names.extend(preload_names(list));
    }
    names.extend(
        RX_CODE_USE
            .captures_iter(source)
            .map(|caps| caps[1].to_string()),
    );
    names.extend(
        RX_TEMPLATE_TAG
            .captures_iter(source)
            .map(|caps| caps[1].to_string()),
    );
    names
}

/// The site component's source file, preferring the readable one.
fn site_source(wc: &Path, name: &str) -> Option<String> {
    let short = name.strip_prefix("app-")?;
    fs::read_to_string(wc.join(format!("{short}.js")))
        .or_else(|_| fs::read_to_string(wc.join(format!("{short}.min.js"))))
        .ok()
}

/// Add everything the site's own components use, following their dependencies all the way down.
pub fn with_site_dependencies(wc: &Path, names: &BTreeSet<String>) -> BTreeSet<String> {
    let mut all = names.clone();
    let mut pending: Vec<String> = names
        .iter()
        .filter(|n| n.starts_with("app-"))
        .cloned()
        .collect();
    while let Some(name) = pending.pop() {
        let Some(source) = site_source(wc, &name) else {
            continue;
        };
        for used in components_in_source(&source) {
            if all.insert(used.clone()) && used.starts_with("app-") {
                pending.push(used);
            }
        }
    }
    all
}

/// The script address the loader requests for a component, or None for a site component that does not exist,
/// such as a data key that only looks like one.
pub fn script_url(wc: &Path, name: &str) -> Option<String> {
    if let Some(short) = name.strip_prefix("app-") {
        return wc
            .join(format!("{short}.min.js"))
            .is_file()
            .then(|| format!("/wc/{short}.min.js"));
    }
    name.strip_prefix("webui-")
        .map(|short| format!("{CDN_COMPONENTS}{short}.min.js"))
}

/// Preload links for the markdown parser and every component the page uses, in a stable order.
pub fn preload_links(wc: &Path, markup: &[&str]) -> String {
    let mut names = BTreeSet::new();
    for text in markup {
        names.extend(components_in_markup(text));
    }
    let names = with_site_dependencies(wc, &names);
    let mut links = vec![format!(
        "\t<link rel=\"modulepreload\" href=\"{MARKDOWN_PARSER}\" crossorigin=\"anonymous\" />"
    )];
    links.extend(names.iter().filter_map(|name| script_url(wc, name)).map(|url| {
        format!("\t<link rel=\"preload\" as=\"script\" href=\"{url}\" crossorigin=\"anonymous\" />")
    }));
    links.join("\n")
}

/// Put links at the end of the head, after the stylesheets and the loader, which matter more.
pub fn add_to_head(html: &str, links: &str) -> String {
    // The markdown parser link is always first, and marks that this tool has already added its links.
    if links.is_empty() || html.contains(&format!("href=\"{MARKDOWN_PARSER}\"")) {
        return html.to_string();
    }
    html.replacen("</head>", &format!("{links}\n</head>"), 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_wc(tag: &str, files: &[(&str, &str)]) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("wpb-preload-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        for (name, body) in files {
            fs::write(dir.join(name), body).unwrap();
        }
        dir
    }

    #[test]
    fn finds_tags_and_preload_attributes_but_not_code_examples() {
        let md = r#"<webui-data data-page-title="T"></webui-data>
<app-json-formatter></app-json-formatter>
<webui-app preload="dialogs app:helper"></webui-app>
Use `<webui-fake>` inline, or:
```html
<webui-example></webui-example>
```
<webui-code lang="html"><webui-shown-as-code></webui-shown-as-code></webui-code>
<!-- <webui-commented></webui-commented> -->
<template><webui-in-template></webui-in-template></template>"#;
        let names: Vec<String> = components_in_markup(md).into_iter().collect();
        assert_eq!(
            names,
            [
                "app-helper",
                "app-json-formatter",
                "webui-app",
                "webui-data",
                "webui-dialogs"
            ]
        );
    }

    #[test]
    fn reads_what_component_source_uses_and_ignores_data_keys() {
        let source = r#"
webui.define('app-tool', {
    preload: 'flex input-message alert',
    render() {
        const out = webui.create('app-tool-output', { label: 'x' });
        webui.setData('app-nav-routes', 1);
        webui.getData('webui-version');
        return Promise.all(['webui-button'].map(tag => customElements.whenDefined('webui-code')));
    },
    shadowTemplate: `<slot></slot><webui-icon icon="x"></webui-icon>`,
});"#;
        let names: Vec<String> = components_in_source(source).into_iter().collect();
        assert_eq!(
            names,
            [
                "app-tool-output",
                "webui-alert",
                "webui-code",
                "webui-flex",
                "webui-icon",
                "webui-input-message"
            ]
        );
    }

    #[test]
    fn follows_site_components_all_the_way_down() {
        let wc = temp_wc(
            "deps",
            &[
                (
                    "json-formatter.js",
                    "webui.define('app-json-formatter', { preload: 'dropdown', x() { webui.create('app-tool-output'); } });",
                ),
                ("json-formatter.min.js", ""),
                (
                    "tool-output.js",
                    "webui.define('app-tool-output', { preload: 'code button' });",
                ),
                ("tool-output.min.js", ""),
            ],
        );
        let start: BTreeSet<String> = ["app-json-formatter".to_string()].into();
        let all: Vec<String> = with_site_dependencies(&wc, &start).into_iter().collect();
        assert_eq!(
            all,
            [
                "app-json-formatter",
                "app-tool-output",
                "webui-button",
                "webui-code",
                "webui-dropdown"
            ]
        );
        let _ = fs::remove_dir_all(&wc);
    }

    #[test]
    fn links_match_the_loader_and_skip_missing_site_components() {
        let wc = temp_wc("links", &[("tool.min.js", "")]);
        let links = preload_links(
            &wc,
            &["<app-tool></app-tool><app-missing></app-missing><webui-flex></webui-flex>"],
        );
        assert_eq!(
            links,
            [
                r#"	<link rel="modulepreload" href="https://cdn.myfi.ws/js/mdparse.min.js" crossorigin="anonymous" />"#,
                r#"	<link rel="preload" as="script" href="/wc/tool.min.js" crossorigin="anonymous" />"#,
                r#"	<link rel="preload" as="script" href="https://cdn.myfi.ws/webui/flex.min.js" crossorigin="anonymous" />"#,
            ]
            .join("\n")
        );
        let _ = fs::remove_dir_all(&wc);
    }

    #[test]
    fn adds_links_to_the_head_once() {
        let html = "<html><head><title>T</title></head><body></body></html>";
        let links = preload_links(Path::new("no-such-wc"), &["<webui-flex></webui-flex>"]);
        let links = links.as_str();
        let once = add_to_head(html, links);
        assert!(once.contains(&format!("{links}\n</head>")));
        assert_eq!(add_to_head(&once, links), once);
    }
}
