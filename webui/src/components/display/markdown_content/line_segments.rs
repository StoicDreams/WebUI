use super::*;
use crate::prelude::regex::*;
use std::collections::HashMap;

const PTN_ANCHOR: &str = r"\[[^\]]+\]\([^\)]+\)";
const PTN_ANCHOR_SEGMENTS: &str =
    r#"\[(?P<display>[^\]]+)\]\((?P<url>[^"\)]+)"?(?P<title>[^"]*)"?\)"#;
const PTN_AVATAR: &str = r"!\[[^\]]*\]\([^\)]+\)";
const PTN_AVATAR_SEGMENTS: &str = r"!\[?(?P<alt>[^\]]+)\]\((?P<url>[^\)]+)\)";
pub(super) fn render_line_segment(segment: &str) -> Html {
    let line_pattern = Regex::new(&format!("^{}$", PTN_AVATAR)).unwrap();
    if line_pattern.is_match(segment) {
        let anchor_pattern = Regex::new(PTN_AVATAR_SEGMENTS).unwrap();
        return match anchor_pattern.captures(segment) {
            Some(caps) => {
                let map: HashMap<&str, &str> = anchor_pattern
                    .capture_names()
                    .flatten()
                    .filter_map(|n| Some((n, caps.name(n)?.as_str())))
                    .collect();
                let alt = map.get("alt").unwrap_or(&"").to_string();
                let url = map.get("url").unwrap_or(&"").trim().to_string();
                let icon = if url.starts_with("fa-") {
                    Some(url.to_owned())
                } else {
                    None
                };
                let image = if url.starts_with("fa-") {
                    None
                } else {
                    Some(url)
                };
                html!(
                    <Avatar {icon} {image} {alt} />
                )
            }
            None => {
                html!({ "ANCHOR PARSE FAILED" })
            }
        };
    }
    let line_pattern = Regex::new(&format!("^{}$", PTN_ANCHOR)).unwrap();
    if line_pattern.is_match(segment) {
        let anchor_pattern = Regex::new(PTN_ANCHOR_SEGMENTS).unwrap();
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
    match get_next_split(segment).as_str() {
        "`" => {
            let subs = segment.split('`');
            let mut is_code = true;
            html!({
                subs.map(|sub| {
                    is_code = !is_code;
                    if is_code {
                        html!(<code>{render_raw_line(sub)}</code>)
                    } else {
                        html!({ render_line_segment(sub) })
                    }
                })
                .collect::<Html>()
            })
        }
        "\"" => {
            let subs = segment.split('\"');
            let mut is_code = true;
            html!({
                subs.map(|sub| {
                    is_code = !is_code;
                    if is_code {
                        html!({ render_raw_line(sub) })
                    } else {
                        html!({ render_line_segment(sub) })
                    }
                })
                .collect::<Html>()
            })
        }
        "**" => {
            let subs = segment.split("**");
            let mut is_strong = true;
            html!({
                subs.map(|sub| {
                    is_strong = !is_strong;
                    if is_strong {
                        html!(<strong>{render_line_segment(sub)}</strong>)
                    } else {
                        html!({ render_line_segment(sub) })
                    }
                })
                .collect::<Html>()
            })
        }
        "==" => {
            let subs = segment.split("==");
            let mut is_highlight = true;
            html!({
                subs.map(|sub| {
                    is_highlight = !is_highlight;
                    if is_highlight {
                        html!(<mark>{render_line_segment(sub)}</mark>)
                    } else {
                        html!({ render_line_segment(sub) })
                    }
                })
                .collect::<Html>()
            })
        }
        "*" => {
            let subs = segment.split('*');
            let mut is_ital = true;
            html!({
                subs.map(|sub| {
                    is_ital = !is_ital;
                    if is_ital {
                        html!(<i>{render_line_segment(sub)}</i>)
                    } else {
                        html!({ render_line_segment(sub) })
                    }
                })
                .collect::<Html>()
            })
        }
        "~" => {
            let subs = segment.split('~');
            let mut is_subscript = true;
            html!({
                subs.map(|sub| {
                    is_subscript = !is_subscript;
                    if is_subscript {
                        html!(<sub>{render_line_segment(sub)}</sub>)
                    } else {
                        html!({ render_line_segment(sub) })
                    }
                })
                .collect::<Html>()
            })
        }
        "^" => {
            let subs = segment.split('^');
            let mut is_super = true;
            html!({
                subs.map(|sub| {
                    is_super = !is_super;
                    if is_super {
                        html!(<sup>{render_line_segment(sub)}</sup>)
                    } else {
                        html!({ render_line_segment(sub) })
                    }
                })
                .collect::<Html>()
            })
        }
        _ => {
            if segment.is_empty() {
                html!()
            } else {
                html!(<span>{ segment }</span>)
            }
        }
    }
}

fn get_next_split(segment: &str) -> String {
    let mut next_split = String::new();
    let mut index = segment.len();
    for item in ["^", "~", "**", "==", "\"", "`"] {
        if let Some(match_index) = index_of(segment, item) {
            if match_index < index {
                index = match_index;
                next_split = item.to_string();
            }
        }
    }
    for item in ["*"] {
        if let Some(match_index) = index_of(segment, item) {
            if match_index < index {
                index = match_index;
                next_split = item.to_string();
            }
        }
    }
    next_split
}

fn index_of(segment: &str, substring: &str) -> Option<usize> {
    let mut subs = segment.split(substring);
    if subs.clone().count() == 1 {
        return None;
    }
    match subs.next() {
        Some(sub) => {
            if !sub.is_empty() {
                return Some(sub.len());
            }
        }
        None => return None,
    }
    match subs.next() {
        Some(sub) => {
            if sub.is_empty() {
                None
            } else {
                Some(0)
            }
        }
        None => None,
    }
}
