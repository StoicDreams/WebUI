use super::*;

pub(super) fn start_render_children(markdown: &Vec<(String, String, MarkdownSegments)>) -> Html {
    let mut index = 1u32;
    html!({ render_children(&mut index, markdown) })
}

pub(super) fn render_children(
    index: &mut u32,
    lines: &Vec<(String, String, MarkdownSegments)>,
) -> Html {
    let mut is_running = true;
    let sec = lines.to_owned();
    let mut counter = 0u32;
    html!({
        lines
            .iter()
            .map(|tuple| {
                counter += 1;
                if counter < *index || !is_running {
                    return html!();
                }
                let mut lines = sec.to_owned();
                let (raw_line, line, line_type) = tuple;
                match line_type {
                    MarkdownSegments::EndSection => {
                        *index += 1;
                        is_running = false;
                        html!()
                    },
                    _ => {
                        if line.is_empty() {
                            *index += 1;
                            return html!();
                        }

                        html!(
                            <>
                                {render_line_content(&mut is_running, line, line_type, index, &mut lines)}
                            </>
                        )
                    }
                }
            })
            .collect::<Html>()
    })
}
