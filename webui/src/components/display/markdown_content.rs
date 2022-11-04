use yew::virtual_dom::VNode;
use core::slice::Iter;
use std::{str::Split, collections::VecDeque};

use crate::*;

/// Properties for Image component
#[derive(Properties, PartialEq)]
pub struct MarkdownContentProps {
    #[prop_or_default]
    pub href: Option<String>,
    #[prop_or_default]
    pub markdown: Option<String>,
}

/// Component for loading and displaying site content from markdown files
///
/// Basic example displaying from url
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		<MarkdownContent href="/d/en-us/example.md"/>
/// 	}
/// }
/// ```
///
/// Apply elevetation
///
/// Basic example displaying from passed in value
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		<MarkdownContent markdown="# Hello World" />
/// 	}
/// }
/// ```
#[function_component(MarkdownContent)]
pub fn site_content(props: &MarkdownContentProps) -> Html {
    let is_loaded = use_state(|| false);
    let markdown = use_state(|| String::from(""));
    match props.markdown.to_owned() {
        Some(md) => {
            markdown.set(md);
        },
        None => {},
    };
    if !*is_loaded && (*markdown).is_empty() {
        match props.href.to_owned() {
            Some(href) => {
                let md = markdown.clone();
                async_std::task::block_on(async move {
                    let response = fetch(FetchRequest::new(href, FetchMethod::Get)).await;
                    if !response.is_ok() {
                        md.set(String::from("Failed to load content."));
                        is_loaded.set(true);
                        return;
                    }
                    match response.get_result() {
                        Some(body) => {
                            md.set(body);
                            is_loaded.set(true);
                        },
                        None => {
                            md.set(String::from("Failed to load content body."));
                            is_loaded.set(true);
                        },
                    }
                });
            },
            None => {},
        }
    }

    let display = markdown_to_html(&*markdown);

    jslog!("Display Markdown Content");
    html! {
        {display}
    }
}

pub fn markdown_to_html(markdown: &str) -> Html {
    let mut lines = Vec::new();
    for line in markdown.lines() {
        lines.push(line);
    }
    html!(
        <>
            {render_lines(&lines)}
        </>
    )
}

fn render_lines(lines: &Vec<&str>) -> Html {
    let mut finished = false;
    let mut lines = lines.iter();
    let segments = &mut Vec::new();
    while !finished {
        match lines.next() {
            Some(line) => {
                segments.push(get_line_type(&line));
                //segments.push(render_segment_getter(*line, &mut lines));
            },
            None => {
                finished = true;
            },
        }
    }
    //segments.reverse();
    jslog!("Lines:{}", segments.len());
    html!(
        <>
            {render_start(segments)}
                // segments.iter().map(|segment|{
            //         html!({segment()})
            //     }).collect::<Html>()
            // }
            // {lines.map(|line| {
            //     let next = line.next();
            //     html!(<p>{line}</p>)
            // }).collect::<Html>()}
            // {
            //     //render_segment("```paper", &mut lines)
            //     // lines.map(move |line| {
            //     //     let lines = sublines;
            //     //     html!(<p>{line}</p>)
            //     // }).collect::<Html>()
            //     // segments.iter().map(|segment|{
            //     // }).collect::<Html>()
            // }
        </>
    )
}

fn render_segment_getter(line: &str, lines: &mut Iter<&str>) -> impl Fn() -> VNode {
    let line = line.to_owned();
    let lines = lines.to_owned();
    let (line, line_type) = get_line_type(&line);
    //let line = line.to_owned();
    //let line_type = line_type.to_owned();
    jslog!("Render Segment:{:?}:{},{}", line_type, line, lines.len());
    move || {
        let line = line.to_owned();
        html!(
            <>
                <p>{line}</p>
            </>
        )
    }
}

fn render_start(lines: &mut Vec<(String, MarkdownSegments)>) -> Html {
    let mut is_running = true;
    html!(
        {lines.iter().map(|(line, line_type)| {
            let mut lines = lines.to_owned();
            let line = line.to_owned();
            let line_type = line_type.to_owned();
            html!(
                <>
                    {match line_type {
                        MarkdownSegments::EndSection => {
                            jslog!("End Section");
                            html!()
                        },
                        MarkdownSegments::Title(level) => {
                            match level {
                                1 => html!(title_primary!(line)),
                                2 => html!(title_secondary!(line)),
                                _ => html!(title_tertiary!(line)),
                            }
                        },
                        MarkdownSegments::Paragraph => {
                            html!(<p>{render_line(&line)}</p>)
                        },
                        MarkdownSegments::PageSection(class, style) => {
                            let class = classes!(CLASSES_PAGE_SECTION, class);
                            html!(<Paper class={class.to_string()} style={style.to_owned()}>
                                {render_start(&mut lines)}
                            </Paper>)
                        },
                        MarkdownSegments::SideImage(src, image_pos, class, style) => {
                            let image_pos = match image_pos.as_str() {
                                "right" => LeftOrRight::Right,
                                _ => LeftOrRight::Left,
                            };
                            html!(<SideImage {image_pos} class={class.to_owned()} src={src.to_owned()} style={style.to_owned()}>
                                {render_start(&mut lines)}
                            </SideImage>)
                        },
                        MarkdownSegments::Paper(class, style) => {
                            html!(<Paper class={class.to_owned()} style={style.to_owned()}>
                                {render_start(&mut lines)}
                            </Paper>)
                        },
                    }}
                </>
            )
        }).collect::<Html>()}
    )
}

fn render_children(lines: &mut Vec<(String, MarkdownSegments)>) -> Html {
    let mut is_running = true;
    let line = lines.pop();
    match line {
        Some((line, line_type)) => {
            jslog!("Render Children:{};{:?}", line, line_type);
            html!(
                <>
                    {match line_type {
                        MarkdownSegments::EndSection => {
                            jslog!("End Section");
                            html!()
                        },
                        MarkdownSegments::Title(level) => {
                            match level {
                                1 => html!(title_primary!(line)),
                                2 => html!(title_secondary!(line)),
                                _ => html!(title_tertiary!(line)),
                            }
                        },
                        MarkdownSegments::Paragraph => {
                            html!(<p>{render_line(&line)}</p>)
                        },
                        MarkdownSegments::PageSection(class, style) => {
                            let class = classes!(CLASSES_PAGE_SECTION, class);
                            html!(<Paper class={class.to_string()} {style}>
                                {render_children(lines)}
                            </Paper>)
                        },
                        MarkdownSegments::SideImage(src, image_pos, class, style) => {
                            let image_pos = match image_pos.as_str() {
                                "right" => LeftOrRight::Right,
                                _ => LeftOrRight::Left,
                            };
                            html!(<SideImage {image_pos} {class} src={src} {style}>
                                {render_children(lines)}
                            </SideImage>)
                        },
                        MarkdownSegments::Paper(class, style) => {
                            html!(<Paper {class} {style}>
                                {render_children(lines)}
                            </Paper>)
                        },
                    }}
                </>
            )
        },
        None => html!()
    }
}

fn render_segment(line: &str, lines: &mut Iter<&str>) -> Html {
    let (line, line_type) = get_line_type(line);
    //let line = line.to_owned();
    //let line_type = line_type.to_owned();
    jslog!("Render Segment:{:?}:{},{}", line_type, line, lines.len());
    html!(
        <>
            {match line_type {
                MarkdownSegments::EndSection => {
                    html!()
                },
                MarkdownSegments::Title(level) => {
                    match level {
                        1 => html!(title_primary!(line)),
                        2 => html!(title_secondary!(line)),
                        _ => html!(title_tertiary!(line)),
                    }
                },
                MarkdownSegments::Paragraph => {
                    html!(<p>{render_line(&line)}</p>)
                },
                MarkdownSegments::PageSection(class, style) => {
                    let class = classes!(CLASSES_PAGE_SECTION, class);
                    let line = (*lines).next().unwrap_or(&"");
                    html!(<Paper class={class.to_string()} {style}>{
                        render_segment(&line, lines)
                    }</Paper>)
                },
                MarkdownSegments::SideImage(src, image_pos, class, style) => {
                    let image_pos = match image_pos.as_str() {
                        "right" => LeftOrRight::Right,
                        _ => LeftOrRight::Left,
                    };
                    let line = (*lines).next().unwrap_or(&"");
                    html!(<SideImage {image_pos} {class} src={src} {style}>
                        {render_segment(&line, lines)}
                    </SideImage>)
                },
                MarkdownSegments::Paper(class, style) => {
                    let line = (*lines).next().unwrap_or(&"");
                    html!(<Paper {class} {style}>
                        {render_segment(&line, lines)}
                    </Paper>)
                },
            }}
        </>
    )
}

fn render_line(line: &str) -> Html {
    html!(
        <>
            {format!("{}", line)}
        </>
    )
}

fn get_line_type(line: &str) -> (String, MarkdownSegments) {
    let mut line = line.to_owned();
    if line.starts_with("###### ") {
        line.replace_range(0..7, "");
        return (line, MarkdownSegments::Title(6));
    }
    if line.starts_with("##### ") {
        line.replace_range(0..6, "");
        return (line, MarkdownSegments::Title(5));
    }
    if line.starts_with("#### ") {
        line.replace_range(0..5, "");
        return (line, MarkdownSegments::Title(4));
    }
    if line.starts_with("### ") {
        line.replace_range(0..4, "");
        return (line, MarkdownSegments::Title(3));
    }
    if line.starts_with("## ") {
        line.replace_range(0..3, "");
        return (line, MarkdownSegments::Title(2));
    }
    if line.starts_with("# ") {
        line.replace_range(0..2, "");
        return (line, MarkdownSegments::Title(1));
    }
    if line.eq("```") {
        return (line, MarkdownSegments::EndSection);
    }
    if line.starts_with("```") {
        while line.starts_with("`") { line.replace_range(0..1, ""); }
        if line.is_empty() { return (line, MarkdownSegments::EndSection); }
        let mut sections = line.split(" ");
        let sectionType = sections.next().unwrap_or("paper").to_lowercase();
        let sections = sections.to_owned().collect::<Vec<&str>>();
        let line = sections.join(" ");
        let mut sections = line.split("\" \"").to_owned();
        let mut test = sections.to_owned();
        return (line.to_owned(), match sectionType.as_str() {
            "section" => MarkdownSegments::PageSection(
                next(&mut sections),
                next(&mut sections),
            ),
            _ => MarkdownSegments::Paper(
                next(&mut sections),
                next(&mut sections),
            )
        });
    }
    (line, MarkdownSegments::Paragraph)

}

fn next(sections: &mut Split<&str>) -> String {
    sections.next().unwrap_or_default().replace("\"", "").to_string()
}

#[derive(Clone,Debug)]
enum MarkdownSegments {
    Title(u8),
    PageSection(String, String),
    Paragraph,
    SideImage(String, String, String, String),
    Paper(String, String),
    EndSection,
}