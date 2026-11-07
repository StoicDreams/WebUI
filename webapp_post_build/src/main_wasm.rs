//! Stop every page from downloading the site's WebAssembly on the main thread.
//!
//! Trunk builds the site's Rust crate and adds a script to index.html that downloads and starts it on the main
//! thread, as `window.wasmBindings`. Web UI sites run their Rust in a web worker instead (`webui.worker`), which
//! loads its own copy (`/webapp.js` and `/webapp_bg.wasm`, built by wasm-pack) only when a page needs it. So the
//! main thread copy, often over a megabyte, was downloaded on every page and never used.
//!
//! This removes Trunk's script, its preload links, and its copy of the files. If any script or page in the build
//! mentions `wasmBindings` or `TrunkApplicationStarted`, the site uses the main thread copy, and it is kept.

use regex::Regex;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

/// Trunk's start script: `import init, * as bindings from '/webapp-{hash}.js'; ... init({ module_or_path: '...wasm' })`.
static RX_TRUNK_SCRIPT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?s)[ \t]*<script type="module">\s*import init, \* as bindings from '(/[^']+\.js)';\s*const wasm = await init\(\{ module_or_path: '(/[^']+\.wasm)' \}\);.*?</script>[ \t]*\r?\n?"#,
    )
    .unwrap()
});
static RX_PRELOAD_LINK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<link rel="(?:modulepreload|preload)" href="([^"]+)"[^>]*>"#).unwrap()
});

/// What a removal found: Trunk's script and the two files it loads.
#[derive(Debug, PartialEq)]
pub struct TrunkStart {
    pub js: String,
    pub wasm: String,
}

/// Find Trunk's start script in index.html.
pub fn find_trunk_start(index: &str) -> Option<TrunkStart> {
    RX_TRUNK_SCRIPT.captures(index).map(|caps| TrunkStart {
        js: caps[1].to_string(),
        wasm: caps[2].to_string(),
    })
}

/// index.html without Trunk's start script and the preload links for its files.
/// Links to `/snippets/` are Trunk's too: the worker imports its snippets itself, without preload links.
pub fn without_trunk_start(index: &str, start: &TrunkStart) -> String {
    let html = RX_TRUNK_SCRIPT.replace(index, "").into_owned();
    // Remove each link with the whitespace before it, so no blank lines are left behind.
    let mut out = String::with_capacity(html.len());
    let mut last = 0;
    for caps in RX_PRELOAD_LINK.captures_iter(&html) {
        let href = &caps[1];
        if href != start.js && href != start.wasm && !href.starts_with("/snippets/") {
            continue;
        }
        let m = caps.get(0).unwrap();
        let before = html[last..m.start()].trim_end_matches([' ', '\t']);
        out.push_str(before);
        last = m.end();
    }
    out.push_str(&html[last..]);
    out
}

/// Whether any script or page mentions the main thread copy, ignoring Trunk's own start script.
pub fn main_copy_is_used(stage: &Path, index_without_start: &str) -> bool {
    let mentions =
        |text: &str| text.contains("wasmBindings") || text.contains("TrunkApplicationStarted");
    if mentions(index_without_start) {
        return true;
    }
    let mut dirs = vec![stage.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
                continue;
            }
            let is_code = matches!(
                path.extension().and_then(|e| e.to_str()),
                Some("js" | "mjs" | "html")
            );
            if is_code
                && path.file_name().and_then(|n| n.to_str()) != Some("index.html")
                && fs::read_to_string(&path).is_ok_and(|text| mentions(&text))
            {
                return true;
            }
        }
    }
    false
}

/// Remove the main thread copy of the WebAssembly from the staged build. Returns true if it was removed.
pub fn run(stage: &Path) -> bool {
    let index_file = stage.join("index.html");
    let Ok(index) = fs::read_to_string(&index_file) else {
        return false;
    };
    let Some(start) = find_trunk_start(&index) else {
        return false;
    };
    let cleaned = without_trunk_start(&index, &start);
    if main_copy_is_used(stage, &cleaned) {
        println!(
            "Main thread WebAssembly: kept, the site uses wasmBindings or TrunkApplicationStarted"
        );
        return false;
    }
    if let Err(err) = fs::write(&index_file, cleaned) {
        println!(
            "Main thread WebAssembly: failed writing {}: {err}",
            index_file.display()
        );
        return false;
    }
    let mut saved = 0;
    for file in [&start.js, &start.wasm] {
        let path = stage.join(file.trim_start_matches('/'));
        saved += fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        let _ = fs::remove_file(&path);
    }
    println!(
        "Main thread WebAssembly: removed {} and {} ({} KB pages no longer download)",
        start.js,
        start.wasm,
        saved / 1024
    );
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const INDEX: &str = r#"<html><head>
	<link rel="stylesheet" href="/app.min.css">
<link rel="modulepreload" href="/webapp-4f38.js" crossorigin="anonymous" integrity="sha384-a"><link rel="modulepreload" href="/snippets/webui-dbe4/src/static_files/js/webui_interop.js" crossorigin="anonymous" integrity="sha384-b">
<link rel="preload" href="/webapp-29fb_bg.wasm" crossorigin="anonymous" integrity="sha384-c" as="fetch" type="application/wasm"></head>
<body><webui-app></webui-app>
<script type="module">
import init, * as bindings from '/webapp-4f38.js';
const wasm = await init({ module_or_path: '/webapp-29fb_bg.wasm' });


window.wasmBindings = bindings;


dispatchEvent(new CustomEvent("TrunkApplicationStarted", {detail: {wasm}}));

</script></body></html>"#;

    fn temp_stage(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("wpb-mainwasm-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("index.html"), INDEX).unwrap();
        fs::write(
            dir.join("webapp-4f38.js"),
            "export default function init() {}",
        )
        .unwrap();
        fs::write(dir.join("webapp-29fb_bg.wasm"), [0u8; 2048]).unwrap();
        // The worker's own copy, which must stay.
        fs::write(dir.join("webapp.js"), "export default function init() {}").unwrap();
        fs::write(dir.join("webapp_bg.wasm"), [0u8; 16]).unwrap();
        dir
    }

    #[test]
    fn finds_and_removes_trunks_start_script_and_links() {
        let start = find_trunk_start(INDEX).unwrap();
        assert_eq!(
            start,
            TrunkStart {
                js: "/webapp-4f38.js".into(),
                wasm: "/webapp-29fb_bg.wasm".into()
            }
        );
        let html = without_trunk_start(INDEX, &start);
        assert!(
            !html.contains("webapp-4f38")
                && !html.contains("webapp-29fb")
                && !html.contains("/snippets/"),
            "{html}"
        );
        assert!(
            !html.contains("wasmBindings") && !html.contains("<script type=\"module\">"),
            "{html}"
        );
        assert!(html.contains(r#"<link rel="stylesheet" href="/app.min.css">"#));
        assert!(html.contains("<webui-app></webui-app>\n</body>"), "{html}");
    }

    #[test]
    fn removes_the_main_copy_and_keeps_the_workers() {
        let stage = temp_stage("remove");
        assert!(run(&stage));
        assert!(
            !stage.join("webapp-4f38.js").exists() && !stage.join("webapp-29fb_bg.wasm").exists()
        );
        assert!(stage.join("webapp.js").exists() && stage.join("webapp_bg.wasm").exists());
        assert!(
            !fs::read_to_string(stage.join("index.html"))
                .unwrap()
                .contains("bindings")
        );
        // A second run finds nothing more to do.
        assert!(!run(&stage));
        let _ = fs::remove_dir_all(&stage);
    }

    #[test]
    fn keeps_the_main_copy_when_the_site_uses_it() {
        let stage = temp_stage("used");
        fs::create_dir_all(stage.join("wc")).unwrap();
        fs::write(
            stage.join("wc").join("app-thing.min.js"),
            "window.wasmBindings.run()",
        )
        .unwrap();
        assert!(!run(&stage));
        assert!(stage.join("webapp-29fb_bg.wasm").exists());
        assert_eq!(fs::read_to_string(stage.join("index.html")).unwrap(), INDEX);
        let _ = fs::remove_dir_all(&stage);
    }
}
