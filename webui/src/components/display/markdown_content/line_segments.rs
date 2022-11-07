use std::collections::HashMap;
use super::*;

const PTN_ANCHOR_SEGMENTS: &str =
    r#"\[(?P<display>[^\]]+)\]\((?P<url>[^ \)]+) ?"?(?P<title>[^"]*)"?\)"#;
pub(super) fn render_line_segment(segment: &str) -> Html {
    let line_pattern = Regex::new(&format!("^{}$", PTN_ANCHOR)).unwrap();
    if line_pattern.is_match(segment) {
        let anchor_pattern = Regex::new(&format!("{}", PTN_ANCHOR_SEGMENTS)).unwrap();
        return match anchor_pattern.captures(segment) {
            Some(caps) => {
                let map: HashMap<&str, &str> = anchor_pattern
                    .capture_names()
                    .flatten()
                    .filter_map(|n| Some((n, caps.name(n)?.as_str())))
                    .collect();
                let display = map.get("display").unwrap_or(&"");
                let url = map.get("url").unwrap_or(&"");
                let title = map.get("title").unwrap_or(&"");
                html!(
                    <Link href={url.to_string()} title={title.to_string()}>
                        {render_line_segment(display)}
                    </Link>
                )
            }
            None => {
                html!({ "ANCHOR PARSE FAILED" })
            }
        };
    }
    if segment.contains("`") {
        let subs = segment.split("`");
        let mut is_code = true;
        return html!({
            subs.map(|sub| {
                is_code = !is_code;
                if is_code {
                    html!(<code>{render_raw_line(sub)}</code>)
                } else {
                    html!({ render_line_segment(sub) })
                }
            })
            .collect::<Html>()
        });
    }
    if segment.contains("**") {
        let subs = segment.split("**");
        let mut is_strong = true;
        return html!({
            subs.map(|sub| {
                is_strong = !is_strong;
                if is_strong {
                    html!(<strong>{render_line_segment(sub)}</strong>)
                } else {
                    html!({ render_line_segment(sub) })
                }
            })
            .collect::<Html>()
        });
    }
    if segment.contains("==") {
        let subs = segment.split("==");
        let mut is_highlight = true;
        return html!({
            subs.map(|sub| {
                is_highlight = !is_highlight;
                if is_highlight {
                    html!(<mark>{render_line_segment(sub)}</mark>)
                } else {
                    html!({ render_line_segment(sub) })
                }
            })
            .collect::<Html>()
        });
    }
    if segment.contains("*") {
        let subs = segment.split("*");
        let mut is_ital = true;
        return html!({
            subs.map(|sub| {
                is_ital = !is_ital;
                if is_ital {
                    html!(<i>{render_line_segment(sub)}</i>)
                } else {
                    html!({ render_line_segment(sub) })
                }
            })
            .collect::<Html>()
        });
    }
    if segment.contains("~") {
        let subs = segment.split("~");
        let mut is_subscript = true;
        return html!({
            subs.map(|sub| {
                is_subscript = !is_subscript;
                if is_subscript {
                    html!(<sub>{render_line_segment(sub)}</sub>)
                } else {
                    html!({ render_line_segment(sub) })
                }
            })
            .collect::<Html>()
        });
    }
    if segment.contains("^") {
        let subs = segment.split("^");
        let mut is_super = true;
        return html!({
            subs.map(|sub| {
                is_super = !is_super;
                if is_super {
                    html!(<sup>{render_line_segment(sub)}</sup>)
                } else {
                    html!({ render_line_segment(sub) })
                }
            })
            .collect::<Html>()
        });
    }
    html!({ segment })
}