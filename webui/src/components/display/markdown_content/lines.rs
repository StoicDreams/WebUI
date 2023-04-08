use std::collections::HashMap;

use super::*;

const PTN_AVATAR: &str = r"(!?\[[^\]]*\]\([^\)]+\))";

pub(super) fn render_raw_line(line: &str) -> Html {
    html!({ line.trim_end() })
}

#[allow(unused_must_use)]
pub(super) fn render_line(line: &str) -> Html {
    let line_pattern = Regex::new(PTN_AVATAR).unwrap();
    let line_segments = line_pattern.captures_iter(line);

    let mut segments_map = HashMap::<usize, (usize, &str)>::new();

    let mut zero_end = 0;
    line_segments
        .map(|segment| {
            segment
                .iter()
                .map(|segment| {
                    if let Some(cap) = segment {
                        let start = cap.start();
                        let end = cap.end();
                        let text = cap.as_str();
                        if text.is_empty() {
                            return;
                        }
                        if !segments_map.contains_key(&start) {
                            segments_map.insert(start, (end, text));
                            if start == 0 && (zero_end == 0 || end < zero_end) {
                                zero_end = end;
                            }
                        } else if start == 0 && end < zero_end {
                            segments_map.remove(&start);
                            segments_map.insert(start, (end, text));
                        }
                    };
                })
                .collect::<()>();
        })
        .collect::<()>();

    let mut segments_list = Vec::new();
    for key in segments_map.keys() {
        segments_list.push((key, segments_map[key]))
    }

    segments_list.sort_unstable_by(|a, b| a.0.cmp(b.0));

    let mut segments = Vec::new();
    let mut next = 0usize;
    for segment in segments_list {
        if *segment.0 > next {
            let slice = line[next..*segment.0].to_string();
            segments.push(slice);
        }
        segments.push(segment.1 .1.to_string());
        next = segment.1 .0;
    }
    if next < line.len() {
        let slice = line[next..line.len()].to_string();
        segments.push(slice);
    }

    html!(
        <>
            {segments.iter().map(|segment| {
                html!({render_line_segment(segment)})
            }).collect::<Html>()}
        </>
    )
}
