use std::{str::Split, collections::HashMap, cmp::Ordering};

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
            },
            None => {
                finished = true;
            },
        }
    }
    let mut index = 0u32;
    html!({render_children(&mut index, segments)})
}

fn render_children(index: &mut u32, lines: &mut Vec<(String, MarkdownSegments)>) -> Html {
    let mut is_running = true;
    let mut sec = lines.to_owned();
    let mut counter = 0u32;
    html!(
        {lines.iter_mut().map(|tuple| {
            counter += 1;
            if counter < *index || !is_running {
                return html!();
            }
            *index += 1;
            let mut lines = sec.to_owned();
            let (line, line_type) = tuple;
            if line.is_empty() { 
                jslog!("RC:Is Empty:{:?}", line_type);
                return html!(); 
            }
            html!(
                <>
                    {render_line_content(&mut is_running, line, line_type, index, &mut lines)}
                </>
            )
        }).collect::<Html>()}
    )
}

fn render_list(index: &mut u32, lines: &mut Vec<(String, MarkdownSegments)>) -> Html {
    let mut is_running = true;
    let mut sec = lines.to_owned();
    let mut counter = 0u32;
    html!(
        {lines.iter_mut().map(|tuple| {
            counter += 1;
            if counter < *index || !is_running {
                return html!();
            }
            *index += 1;
            let mut lines = sec.to_owned();
            let (line, line_type) = tuple;
            if line.is_empty() {
                jslog!("RL:Is Empty:{:?}", line_type);
                return html!();
            }
            html!(
                <li>
                    {render_line_content(&mut is_running, line, line_type, index, &mut lines)}
                </li>
            )
        }).collect::<Html>()}
    )
}

fn render_line_content(is_running: &mut bool, line: &str, line_type: &MarkdownSegments, index: &mut u32, lines: &mut Vec<(String, MarkdownSegments)>) -> Html {
    html!(
        <>
        {match line_type {
            MarkdownSegments::EndSection => {
                *is_running = false;
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
                *index += 1;
                let class = classes!(CLASSES_PAGE_SECTION, class);
                html!(<Paper class={class.to_string()} style={style.to_owned()}>
                    {render_children(index, lines)}
                </Paper>)
            },
            MarkdownSegments::SideImage(image_pos, src, class, style) => {
                *index += 1;
                let image_pos = match image_pos.as_str() {
                    "right" => LeftOrRight::Right,
                    _ => LeftOrRight::Left,
                };
                html!(<SideImage {image_pos} class={class.to_owned()} src={src.to_owned()} style={style.to_owned()}>
                    <Paper>{render_children(index, lines)}</Paper>
                </SideImage>)
            },
            MarkdownSegments::Paper(class, style) => {
                *index += 1;
                html!(<Paper class={class.to_owned()} style={style.to_owned()}>
                    {render_children(index, lines)}
                </Paper>)
            },
            MarkdownSegments::List(is_ordered) => {
                *index += 1;
                html!(<List>
                    {render_list(index, lines)}
                </List>)
            },
            MarkdownSegments::Cards(class, style) => {
                *index += 1;
                jslog!("Display Cards");
                let class = classes!(CLASSES_CARD_CONTAINER, class);
                html!(<Paper class={class.to_string()} style={style.to_owned()}>
                    {render_children(index, lines)}
                </Paper>)
            },
            MarkdownSegments::Card(title, width, theme, avatar, link) => {
                *index += 1;
                let theme = match theme.as_str() {
                    "active" => Theme::Active,
                    "background" => Theme::Background,
                    "black" => Theme::Black,
                    "white" => Theme::White,
                    "secondary" => Theme::Secondary,
                    "tertiary" => Theme::Tertiary,
                    "info" => Theme::Info,
                    "success" => Theme::Success,
                    "warning" => Theme::Warning,
                    "danger" => Theme::Danger,
                    "title" => Theme::Title,
                    "inherit" => Theme::None,
                    _ => Theme::Primary,
                };
                html!(<Card title={title.to_owned()}
                    width={*width}
                    theme={theme}
                    avatar={avatar.to_owned()}
                    link={link.to_owned()}
                    elevation={ELEVATION_STANDARD}
                    >
                    {render_children(index, lines)}
                </Card>)
            },
        }}
        </>
    )
}

const PTN_NON_START_BRACKET: &str = r"([^\[]*)";
const PTN_ANCHOR: &str = r"(\[[^\]]+\]\([^\)]+\))";

fn render_line(line: &str) -> Html {
    let line_pattern = Regex::new(
        &format!("{}?{}?{}?{}?{}?{}?{}?{}?{}?{}?{}?{}?{}?",
        PTN_NON_START_BRACKET, PTN_ANCHOR,
        PTN_NON_START_BRACKET, PTN_ANCHOR,
        PTN_NON_START_BRACKET, PTN_ANCHOR,
        PTN_NON_START_BRACKET, PTN_ANCHOR,
        PTN_NON_START_BRACKET, PTN_ANCHOR,
        PTN_NON_START_BRACKET, PTN_ANCHOR,
        PTN_NON_START_BRACKET,
    )).unwrap();
    let line_segments = line_pattern.captures_iter(line);
    
    let mut segments_map = HashMap::<usize, &str>::new();

    let mut zero_end = 0;
    _ = line_segments.map(|segment| {
        _ = segment.iter().map(|segment| {
            match segment {
                Some(cap) => {
                    let start = cap.start();
                    let end = cap.end();
                    let text = cap.as_str();
                    if text.is_empty() { return; }
                    if !segments_map.contains_key(&start) {
                        segments_map.insert(start, text);
                        if start == 0 && (zero_end == 0 || end < zero_end) {
                            zero_end = end;
                        }
                    } else if start == 0 && end < zero_end {
                        segments_map.remove(&start);
                        segments_map.insert(start, text);
                    }
                },
                None => ()
            };
            ()
        }).collect::<()>();
        ()
    }).collect::<()>();

    let mut segments_list = Vec::new();
    for key in segments_map.keys() {
        segments_list.push((key, segments_map[key]))
    }

    segments_list.sort_unstable_by(|a, b| {
        if a.0 < b.0 { Ordering::Less }
        else if b.0 < a.0 { Ordering::Greater }
        else { Ordering::Equal }
    });
    
    html!(
        <>
            {segments_list.iter().map(|segment| {
                html!({render_line_segment(segment.1)})
            }).collect::<Html>()}
        </>
    )
}

const PTN_ANCHOR_SEGMENTS: &str = r#"\[(?P<display>[^\]]+)\]\((?P<url>[^ \)]+) ?"?(?P<title>[^"]*)"?\)"#;
fn render_line_segment(segment: &str) -> Html {
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
                        {display}
                    </Link>
                )
            },
            None => {
                html!({"ANCHOR PARSE FAILED"})
            }
        };
    }
    html!({segment})
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
        let section_type = sections.next().unwrap_or("paper").to_lowercase();
        let sections = sections.to_owned().collect::<Vec<&str>>();
        let line = sections.join(" ");
        let mut sections = line.split("\" \"").to_owned();
        let mut test = sections.to_owned();
        jslog!("SectionType: {}", section_type.as_str());
        return (section_type.to_string(), match section_type.as_str() {
            "cards" => MarkdownSegments::Cards(
                next(&mut sections),
                next(&mut sections),
            ),
            "card" => MarkdownSegments::Card(
                next(&mut sections),
                u16::from_str(&next(&mut sections)).unwrap_or(800u16),
                next(&mut sections),
                next(&mut sections),
                next(&mut sections),
            ),
            "list" => MarkdownSegments::List(
                !next(&mut sections).is_empty(),
            ),
            "section" => MarkdownSegments::PageSection(
                next(&mut sections),
                next(&mut sections),
            ),
            "sideimage" => MarkdownSegments::SideImage(
                next(&mut sections),
                next(&mut sections),
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
    Cards(String, String),
    Card(String, u16, String, String, String),
    List(bool),
    EndSection,
}