use super::*;

pub(super) fn render_code_segments(
    index: &mut u32,
    lines: &mut Vec<(String, String, MarkdownSegments)>,
) -> Html {
    let mut is_running = true;
    let mut sec = lines.to_owned();
    let mut counter = 0u32;
    html!({
        lines
            .iter_mut()
            .map(|tuple| {
                counter += 1;
                if counter < *index || !is_running {
                    return html!();
                }
                let mut lines = sec.to_owned();
                let (raw_line, line, line_type) = tuple;
                *index += 1;
                match line_type {
                    MarkdownSegments::EndSection => {
                        is_running = false;
                        html!()
                    }
                    _ => {
                        html!(<p>{render_raw_line(raw_line)}</p>)
                    }
                }
            })
            .collect::<Html>()
    })
}
