//! Write a static HTML file for each page, so search engines and link previews see the page's real
//! title, description, and text without running JavaScript.
//!
//! Each file is a copy of the built `index.html` with the page's details and content filled in.
//! The app replaces the content with its own rendering once it loads, so visitors see no difference.

use pulldown_cmark::{Options, Parser, html};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

/// Pages longer than this are cut at a word boundary for the meta description.
const DESCRIPTION_LENGTH: usize = 160;

/// Marks the rule this tool adds to web.config, so it is only added once.
const WEB_CONFIG_RULE_NAME: &str = "Prerendered Pages";

static RX_WEBUI_DATA: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?is)<webui-data\b[^>]*>").unwrap());
static RX_ATTR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?is)([a-z0-9-]+)\s*=\s*(?:"([^"]*)"|'([^']*)')"#).unwrap());
static RX_CUSTOM_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?is)</?(?:webui|app)-[a-z0-9-]+\b[^>]*>").unwrap());
static RX_TAG_LINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^(?:</?(?:webui|app)-[a-z0-9-]+\b[^>]*>\s*)+$").unwrap());
static RX_EMPTY_PARAGRAPH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)<p>\s*</p>\s*").unwrap());
static RX_SCRIPT_STYLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?is)<(script|style|template)\b[^>]*>.*?</(?:script|style|template)>").unwrap()
});
static RX_PARAGRAPH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?is)<p>(.*?)</p>").unwrap());
static RX_ANY_TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)<[^>]+>").unwrap());
static RX_SPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());
static RX_TOKEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{([^{}\s]+)\}").unwrap());
static RX_TITLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?is)<title>.*?</title>").unwrap());
static RX_OG_URL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?is)<meta\s+property="og:url"\s+content="([^"]+)""#).unwrap());
static RX_CANONICAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?is)\s*<link\s+rel="canonical"[^>]*>"#).unwrap());
static RX_HEADER_TITLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?is)(<h1\s+data-subscribe="page-title:innerHTML"[^>]*>).*?(</h1>)"#).unwrap()
});
static RX_HEADER_SUBTITLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?is)(<h2\s+data-subscribe="page-subtitle:innerHTML"[^>]*>).*?(</h2>)"#).unwrap()
});
// Must not match <webui-app-config>, which comes first in index.html.
static RX_APP_OPEN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?is)<webui-app(?:\s[^>]*)?>").unwrap());

/// Details read from a page's markdown.
#[derive(Debug, Default, PartialEq)]
pub struct Page {
    pub route: String,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub content: String,
}

/// Write a prerendered file for every page in `{stage}/d`. Returns the number of pages written.
pub fn run(stage: &Path) -> usize {
    let index_file = stage.join("index.html");
    let Ok(index) = fs::read_to_string(&index_file) else {
        println!("Prerender: skipped, no index.html in {}", stage.display());
        return 0;
    };
    let Some(origin) = site_origin(&index) else {
        println!("Prerender: skipped, index.html has no og:url to build page addresses from");
        return 0;
    };
    let config = read_json(&stage.join("appConfig.json"));
    let tokens = data_tokens(&config, &origin);
    // The app fills in placeholders such as the footer's "About {APP_NAME}" when it loads. Fill them in for crawlers.
    let index = replace_tokens(&index, &tokens);
    let dynamic: Vec<String> = config
        .get("dynNavRoutes")
        .and_then(Value::as_array)
        .map(|routes| {
            routes
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_lowercase)
                .collect()
        })
        .unwrap_or_default();

    let pages_dir = stage.join("d");
    let mut written = 0;
    for file in markdown_files(&pages_dir) {
        let route = route_for(&pages_dir, &file);
        if dynamic
            .iter()
            .any(|prefix| route.starts_with(prefix.as_str()))
        {
            continue;
        }
        let Ok(markdown) = fs::read_to_string(&file) else {
            continue;
        };
        let page = parse_page(&route, &markdown, &tokens);
        let target = if route == "/" {
            index_file.clone()
        } else {
            stage.join(format!("{}.html", route.trim_start_matches('/')))
        };
        if route != "/" && target.exists() {
            println!(
                "Prerender: skipped {route}, {} already exists",
                target.display()
            );
            continue;
        }
        // Hints for the components this page and the site's shell use, so they load alongside the framework.
        let links = crate::preload::preload_links(&stage.join("wc"), &[&index, &markdown]);
        let html = crate::preload::add_to_head(&render_page(&index, &page, &origin), &links);
        if let Some(parent) = target.parent() {
            let _ = fs::create_dir_all(parent);
        }
        match fs::write(&target, html) {
            Ok(_) => written += 1,
            Err(err) => println!("Prerender: failed writing {}: {err}", target.display()),
        }
    }
    if written > 0 {
        add_web_config_rule(&stage.join("web.config"));
    }
    println!("Prerender: wrote {written} pages");
    written
}

fn read_json(file: &Path) -> Value {
    fs::read_to_string(file)
        .ok()
        .and_then(|text| serde_json::from_str(text.trim_start_matches('\u{feff}')).ok())
        .unwrap_or(Value::Null)
}

/// The site's own origin, such as `https://www.example.com`, from the og:url in index.html.
pub fn site_origin(index: &str) -> Option<String> {
    let url = RX_OG_URL.captures(index)?.get(1)?.as_str();
    let rest = url.split_once("://")?;
    let host = rest.1.split('/').next()?;
    Some(format!("{}://{}", rest.0, host))
}

fn markdown_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return files;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(markdown_files(&path));
        } else if path.extension().is_some_and(|ext| ext == "md") {
            files.push(path);
        }
    }
    files.sort();
    files
}

/// The address of a page file, matching how the app loads pages: `d/about.md` is `/about`, `d/root.md` is `/`.
pub fn route_for(pages_dir: &Path, file: &Path) -> String {
    let relative = file
        .strip_prefix(pages_dir)
        .unwrap_or(file)
        .with_extension("");
    let parts: Vec<String> = relative
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_lowercase())
        .collect();
    let route = parts.join("/");
    if route == "root" {
        "/".to_string()
    } else {
        format!("/{route}")
    }
}

/// Placeholders such as `{APP_NAME}` and `{COMPANY_SINGULAR}`, matching the app's own replacements.
pub fn data_tokens(config: &Value, origin: &str) -> HashMap<String, String> {
    let mut tokens = HashMap::new();
    let mut add = |key: &str, value: String| {
        let snake = to_snake(key).to_uppercase();
        if let Some(short) = snake.strip_prefix("APP_") {
            tokens
                .entry(short.to_string())
                .or_insert_with(|| value.clone());
        }
        tokens.entry(snake).or_insert(value);
    };
    if let Some(map) = config.as_object() {
        for (key, value) in map {
            if let Some(text) = value.as_str() {
                add(key, text.to_string());
            }
        }
    }
    let domain = origin.split("://").nth(1).unwrap_or(origin).to_lowercase();
    add("appDomain", domain);
    tokens
}

fn to_snake(name: &str) -> String {
    let mut out = String::new();
    for (index, ch) in name.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if index > 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else if ch == '-' {
            out.push('_');
        } else {
            out.push(ch);
        }
    }
    out
}

pub fn replace_tokens(text: &str, tokens: &HashMap<String, String>) -> String {
    RX_TOKEN
        .replace_all(text, |caps: &regex::Captures| {
            tokens
                .get(&caps[1])
                .cloned()
                .unwrap_or_else(|| caps[0].to_string())
        })
        .into_owned()
}

/// Read a page's title, subtitle, description, and plain HTML content from its markdown.
pub fn parse_page(route: &str, markdown: &str, tokens: &HashMap<String, String>) -> Page {
    let markdown = replace_tokens(&markdown.replace("\r\n", "\n"), tokens);
    let mut page = Page {
        route: route.to_string(),
        ..Default::default()
    };
    if let Some(tag) = RX_WEBUI_DATA.find(&markdown) {
        for caps in RX_ATTR.captures_iter(tag.as_str()) {
            let value = caps
                .get(2)
                .or(caps.get(3))
                .map(|m| decode_entities(m.as_str()))
                .unwrap_or_default();
            match caps[1].to_lowercase().as_str() {
                "data-page-title" => page.title = value,
                "data-page-subtitle" => page.subtitle = value,
                "data-page-description" => page.description = value,
                _ => {}
            }
        }
    }
    page.content = markdown_to_plain_html(&markdown);
    if page.title.is_empty() {
        page.title = title_from_route(route);
    }
    if page.description.is_empty() {
        page.description = first_paragraph_text(&page.content);
    }
    page
}

/// Convert page markdown to HTML without the framework's components, so the result loads nothing and runs nothing.
pub fn markdown_to_plain_html(markdown: &str) -> String {
    // The app's parser reads markdown inside component tags and ignores indentation, while CommonMark treats
    // indented lines as code and text right after a tag as raw HTML. Match the app by trimming lines and putting
    // component tags on lines of their own, outside fenced code.
    let mut prepared = String::new();
    let mut in_fence = false;
    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
        }
        if in_fence || trimmed.starts_with("```") {
            prepared.push_str(line);
            prepared.push('\n');
        } else if RX_TAG_LINE.is_match(trimmed) || RX_WEBUI_DATA.is_match(trimmed) {
            prepared.push('\n');
            prepared.push_str(trimmed);
            prepared.push_str("\n\n");
        } else {
            prepared.push_str(trimmed);
            prepared.push('\n');
        }
    }
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let mut rendered = String::new();
    html::push_html(&mut rendered, Parser::new_ext(&prepared, options));
    let rendered = RX_SCRIPT_STYLE.replace_all(&rendered, "");
    let rendered = RX_CUSTOM_TAG.replace_all(&rendered, "");
    let rendered = RX_EMPTY_PARAGRAPH.replace_all(&rendered, "");
    rendered.trim().to_string()
}

fn title_from_route(route: &str) -> String {
    let last = route.rsplit('/').next().unwrap_or("");
    let words: Vec<String> = last
        .split(['-', '_'])
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut chars = w.chars();
            chars
                .next()
                .map(|c| c.to_uppercase().collect::<String>() + chars.as_str())
                .unwrap_or_default()
        })
        .collect();
    words.join(" ")
}

/// The first paragraph of text, shortened to fit a meta description.
pub fn first_paragraph_text(content: &str) -> String {
    for caps in RX_PARAGRAPH.captures_iter(content) {
        let text = RX_ANY_TAG.replace_all(&caps[1], " ");
        let text = decode_entities(RX_SPACE.replace_all(&text, " ").trim());
        if text.chars().count() >= 20 {
            return shorten(&text, DESCRIPTION_LENGTH);
        }
    }
    String::new()
}

fn shorten(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_string();
    }
    let cut: String = text.chars().take(max - 1).collect();
    let cut = match cut.rfind(' ') {
        Some(space) if space > max / 2 => &cut[..space],
        _ => cut.as_str(),
    };
    format!("{}…", cut.trim_end_matches([',', '.', ';', ':', ' ']))
}

fn decode_entities(text: &str) -> String {
    text.replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&#x2F;", "/")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// The browser title for a page, built the same way the app builds it once it loads: "Title: Subtitle".
/// The home page keeps the title written in index.html.
pub fn full_title(page: &Page) -> String {
    [page.title.as_str(), page.subtitle.as_str()]
        .iter()
        .filter(|part| !part.is_empty())
        .cloned()
        .collect::<Vec<_>>()
        .join(": ")
}

/// Fill in the built index.html with one page's details and content.
pub fn render_page(index: &str, page: &Page, origin: &str) -> String {
    let is_root = page.route == "/";
    let url = if is_root {
        format!("{origin}/")
    } else {
        format!("{origin}{}", page.route)
    };
    let mut html = index.to_string();
    if !is_root {
        let title = escape(&full_title(page));
        html = RX_TITLE
            .replace(&html, format!("<title>{title}</title>").as_str())
            .into_owned();
        html = set_meta(&html, "property", "og:title", &title);
        html = set_meta(&html, "name", "twitter:title", &title);
        html = set_meta(&html, "property", "og:url", &escape(&url));
        html = set_meta(&html, "name", "twitter:url", &escape(&url));
        if !page.description.is_empty() {
            let description = escape(&page.description);
            html = set_meta(&html, "name", "description", &description);
            html = set_meta(&html, "property", "og:description", &description);
            html = set_meta(&html, "name", "twitter:description", &description);
        }
        // index.html is also served for addresses without a file of their own, so only pages get a canonical link.
        html = RX_CANONICAL.replace_all(&html, "").into_owned();
        html = html.replacen(
            "</head>",
            &format!(
                "\t<link rel=\"canonical\" href=\"{}\" />\n</head>",
                escape(&url)
            ),
            1,
        );
    }
    let title = escape(&page.title);
    let subtitle = escape(&page.subtitle);
    html = RX_HEADER_TITLE
        .replace(&html, |caps: &regex::Captures| {
            format!("{}{title}{}", &caps[1], &caps[2])
        })
        .into_owned();
    html = RX_HEADER_SUBTITLE
        .replace(&html, |caps: &regex::Captures| {
            format!("{}{subtitle}{}", &caps[1], &caps[2])
        })
        .into_owned();
    if let Some(open) = RX_APP_OPEN.find(&html) {
        let insert = format!("\n\t\t<div data-prerendered>\n{}\n\t\t</div>", page.content);
        html.insert_str(open.end(), &insert);
    }
    html
}

fn set_meta(html: &str, attr: &str, key: &str, value: &str) -> String {
    let pattern = format!(
        r#"(?is)(<meta\s+{attr}="{}"\s+content=")[^"]*(")"#,
        regex::escape(key)
    );
    let rx = Regex::new(&pattern).unwrap();
    rx.replace(html, |caps: &regex::Captures| {
        format!("{}{value}{}", &caps[1], &caps[2])
    })
    .into_owned()
}

/// Serve `/about` from `about.html` on IIS. Added before the rule that sends every other address to index.html.
pub fn add_web_config_rule(file: &Path) {
    let Ok(config) = fs::read_to_string(file) else {
        return;
    };
    if let Some(updated) = web_config_with_rule(&config)
        && fs::write(file, updated).is_ok()
    {
        println!("Prerender: added the {WEB_CONFIG_RULE_NAME} rule to web.config");
    }
}

pub fn web_config_with_rule(config: &str) -> Option<String> {
    if config.contains(WEB_CONFIG_RULE_NAME) {
        return None;
    }
    let spa = config.find(r#"<rule name="Rewrite To SPA""#)?;
    let line_start = config[..spa].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let indent = &config[line_start..spa];
    let newline = if config.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let rule = [
        format!(r#"<rule name="{WEB_CONFIG_RULE_NAME}" stopProcessing="true">"#),
        r#"    <match url="^(.+?)/?$" />"#.to_string(),
        "    <conditions>".to_string(),
        r#"        <add input="{REQUEST_FILENAME}" matchType="IsFile" negate="true" />"#
            .to_string(),
        r#"        <add input="{REQUEST_FILENAME}.html" matchType="IsFile" />"#.to_string(),
        "    </conditions>".to_string(),
        r#"    <action type="Rewrite" url="{R:1}.html" />"#.to_string(),
        "</rule>".to_string(),
    ]
    .iter()
    .map(|line| format!("{indent}{line}"))
    .collect::<Vec<_>>()
    .join(newline);
    let mut updated = config.to_string();
    updated.insert_str(line_start, &format!("{rule}{newline}"));
    Some(updated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn tokens() -> HashMap<String, String> {
        data_tokens(
            &json!({ "appName": "Ap Tul", "appCompanySingular": "Stoic Dreams" }),
            "https://www.aptul.com",
        )
    }

    #[test]
    fn placeholders_match_the_app() {
        let t = tokens();
        assert_eq!(
            replace_tokens("{APP_NAME} by {COMPANY_SINGULAR}", &t),
            "Ap Tul by Stoic Dreams"
        );
        assert_eq!(
            replace_tokens("{NAME} on {APP_DOMAIN}", &t),
            "Ap Tul on www.aptul.com"
        );
        assert_eq!(replace_tokens("{UNKNOWN} stays", &t), "{UNKNOWN} stays");
    }

    #[test]
    fn routes_match_page_files() {
        let dir = Path::new("d");
        assert_eq!(route_for(dir, Path::new("d/root.md")), "/");
        assert_eq!(route_for(dir, Path::new("d/About.md")), "/about");
        assert_eq!(
            route_for(dir, Path::new("d/blogs/code-reviews.md")),
            "/blogs/code-reviews"
        );
    }

    #[test]
    fn reads_page_data_and_description() {
        let md = r#"<webui-data data-page-title="About {APP_NAME}" data-page-subtitle="Sub &amp; more"></webui-data>

<webui-page-segment elevation="10">
This is the **first** paragraph, with enough words to be a description.
</webui-page-segment>
"#;
        let page = parse_page("/about", md, &tokens());
        assert_eq!(page.title, "About Ap Tul");
        assert_eq!(page.subtitle, "Sub & more");
        assert_eq!(
            page.description,
            "This is the first paragraph, with enough words to be a description."
        );
        assert!(
            page.content.contains("<strong>first</strong>"),
            "{}",
            page.content
        );
        assert!(!page.content.contains("webui-"), "{}", page.content);
    }

    #[test]
    fn description_override_and_title_fallback() {
        let md = r#"<webui-data data-page-description="Custom text"></webui-data>"#;
        let page = parse_page("/image-to-base64", md, &tokens());
        assert_eq!(page.title, "Image To Base64");
        assert_eq!(page.description, "Custom text");
    }

    #[test]
    fn indented_markdown_inside_components_is_not_code() {
        let html = markdown_to_plain_html(
            "<webui-page-segment>\n    Indented text inside a tag.\n</webui-page-segment>",
        );
        assert_eq!(html, "<p>Indented text inside a tag.</p>");
    }

    #[test]
    fn fenced_code_keeps_its_content() {
        let html = markdown_to_plain_html("```pwsh\n    indented <webui-x>\n```");
        assert!(html.contains("    indented &lt;webui-x&gt;"), "{html}");
    }

    #[test]
    fn removes_scripts_and_components() {
        let html = markdown_to_plain_html(
            "<app-image-converter></app-image-converter>\n\n<script>alert(1)</script>\n\nText here",
        );
        assert_eq!(html, "<p>Text here</p>");
    }

    #[test]
    fn long_descriptions_are_cut_at_a_word() {
        let text = "word ".repeat(60);
        let short = shorten(text.trim(), DESCRIPTION_LENGTH);
        assert!(short.chars().count() <= DESCRIPTION_LENGTH);
        assert!(short.ends_with("word…"), "{short}");
    }

    const INDEX: &str = r#"<html><head><title>Ap Tul | Toolkit</title>
<meta name="description" content="Site description">
<meta property="og:url" content="https://www.aptul.com/">
<meta property="og:title" content="Ap Tul">
<meta name="twitter:description" content="Site description">
</head><body><webui-app-config src="appConfig.json"></webui-app-config>
<webui-app data-x="1"><header slot="header"><h1 data-subscribe="page-title:innerHTML">Ap Tul</h1><h2 data-subscribe="page-subtitle:innerHTML"></h2></header></webui-app></body></html>"#;

    #[test]
    fn page_files_get_their_own_details() {
        let page = Page {
            route: "/jwt-parser".into(),
            title: "JWT Parser".into(),
            subtitle: "Decode & inspect".into(),
            description: "Decode tokens \"safely\"".into(),
            content: "<p>Body</p>".into(),
        };
        let html = render_page(INDEX, &page, "https://www.aptul.com");
        assert!(html.contains("<title>JWT Parser: Decode &amp; inspect</title>"));
        assert!(
            html.contains(
                r#"<meta name="description" content="Decode tokens &quot;safely&quot;">"#
            )
        );
        assert!(html.contains(
            r#"<meta name="twitter:description" content="Decode tokens &quot;safely&quot;">"#
        ));
        assert!(
            html.contains(r#"<meta property="og:url" content="https://www.aptul.com/jwt-parser">"#)
        );
        assert!(
            html.contains(r#"<link rel="canonical" href="https://www.aptul.com/jwt-parser" />"#)
        );
        assert!(html.contains(">JWT Parser</h1>"));
        assert!(html.contains(">Decode &amp; inspect</h2>"));
        assert!(html.contains("<webui-app data-x=\"1\">\n\t\t<div data-prerendered>\n<p>Body</p>"));
        assert!(
            html.contains("<webui-app-config src=\"appConfig.json\"></webui-app-config>"),
            "content must not go in webui-app-config"
        );
    }

    #[test]
    fn home_page_keeps_its_title_and_has_no_canonical() {
        let page = Page {
            route: "/".into(),
            title: "Ap Tul".into(),
            content: "<p>Home</p>".into(),
            ..Default::default()
        };
        let html = render_page(INDEX, &page, "https://www.aptul.com");
        assert!(html.contains("<title>Ap Tul | Toolkit</title>"));
        assert!(!html.contains("canonical"));
        assert!(html.contains("<p>Home</p>"));
    }

    #[test]
    fn titles_match_the_app() {
        let page = Page {
            title: "About Ap Tul".into(),
            ..Default::default()
        };
        assert_eq!(full_title(&page), "About Ap Tul");
        let page = Page {
            title: "Image Converter".into(),
            subtitle: "Resize & convert".into(),
            ..Default::default()
        };
        assert_eq!(full_title(&page), "Image Converter: Resize & convert");
    }

    #[test]
    fn web_config_rule_goes_before_the_spa_rule_once() {
        let config = "<rules>\n                <rule name=\"Rewrite To SPA\" stopProcessing=\"true\">\n                </rule>\n</rules>";
        let updated = web_config_with_rule(config).unwrap();
        let ours = updated.find(WEB_CONFIG_RULE_NAME).unwrap();
        let spa = updated.find("Rewrite To SPA").unwrap();
        assert!(ours < spa);
        assert!(
            updated.contains("                <rule name=\"Prerendered Pages\""),
            "{updated}"
        );
        assert!(
            updated.contains("                    <action type=\"Rewrite\" url=\"{R:1}.html\" />"),
            "{updated}"
        );
        assert!(web_config_with_rule(&updated).is_none());
    }
}
