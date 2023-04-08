use super::*;
use std::str::Split;

const SECTION_DELIM: &str = " ";
pub(super) fn get_line_type(line: &str) -> (String, String, MarkdownSegments) {
    let mut line = line.to_owned();
    let raw_line = line.to_string();
    if line.starts_with(">loading") {
        line.replace_range(0..1, EMPTY_STRING);
        let mut sections = line.split(SECTION_DELIM);
        let section_type = get_section_type(&mut sections);
        let binding = get_binding(sections);
        let mut sections = binding.split("\" \"");
        return (
            raw_line,
            section_type,
            MarkdownSegments::Loading(
                get_loading_variant(&vnext(&mut sections)),
                get_theme(&vnext(&mut sections)),
                get_u16(&vnext(&mut sections)),
                get_option(&vnext(&mut sections)).map(get_u8),
                vnext(&mut sections),
                vnext(&mut sections),
            ),
        );
    }
    if line.starts_with("###### ") {
        line.replace_range(0..7, EMPTY_STRING);
        return (raw_line, line, MarkdownSegments::Title(6));
    }
    if line.starts_with("##### ") {
        line.replace_range(0..6, EMPTY_STRING);
        return (raw_line, line, MarkdownSegments::Title(5));
    }
    if line.starts_with("#### ") {
        line.replace_range(0..5, EMPTY_STRING);
        return (raw_line, line, MarkdownSegments::Title(4));
    }
    if line.starts_with("### ") {
        line.replace_range(0..4, EMPTY_STRING);
        return (raw_line, line, MarkdownSegments::Title(3));
    }
    if line.starts_with("## ") {
        line.replace_range(0..3, EMPTY_STRING);
        return (raw_line, line, MarkdownSegments::Title(2));
    }
    if line.starts_with("# ") {
        line.replace_range(0..2, EMPTY_STRING);
        return (raw_line, line, MarkdownSegments::Title(1));
    }
    if line.eq("```") {
        return (raw_line, line, MarkdownSegments::EndSection);
    }
    if line.starts_with("```") {
        while line.starts_with('`') {
            line.replace_range(0..1, "");
        }
        if line.is_empty() {
            return (raw_line, line, MarkdownSegments::EndSection);
        }
        let mut sections = line.split(SECTION_DELIM);
        let section_type = get_section_type(&mut sections);
        let binding = get_binding(sections);
        let mut sections = binding.split("\" \"");
        return (
            raw_line,
            section_type.to_string(),
            match section_type.as_str() {
                "automax" | "automaxcontent" => {
                    MarkdownSegments::AutoMaxContent(vnext(&mut sections), vnext(&mut sections))
                }
                "cards" => MarkdownSegments::Cards(vnext(&mut sections), vnext(&mut sections)),
                "card" => MarkdownSegments::Card(
                    vnext(&mut sections),
                    u16::from_str(&vnext(&mut sections)).unwrap_or(800u16),
                    vnext(&mut sections),
                    vnext(&mut sections),
                    vnext(&mut sections),
                    vnext(&mut sections),
                ),
                "list" => MarkdownSegments::List(
                    !vnext(&mut sections).is_empty(),
                    vnext(&mut sections),
                    vnext(&mut sections),
                ),
                "maxauto" | "maxcontentauto" => {
                    MarkdownSegments::MaxContentAuto(vnext(&mut sections), vnext(&mut sections))
                }
                "quote" => MarkdownSegments::Quote(
                    vnext(&mut sections),
                    vnext(&mut sections),
                    vnext(&mut sections),
                    vnext(&mut sections),
                ),
                "section" => {
                    MarkdownSegments::PageSection(vnext(&mut sections), vnext(&mut sections))
                }
                "sidebyside" => {
                    MarkdownSegments::SideBySide(vnext(&mut sections), vnext(&mut sections))
                }
                "sideimage" => MarkdownSegments::SideImage(
                    vnext(&mut sections),
                    vnext(&mut sections),
                    vnext(&mut sections),
                    vnext(&mut sections),
                ),
                "paper" => MarkdownSegments::Paper(vnext(&mut sections), vnext(&mut sections)),
                _ => {
                    MarkdownSegments::Code(section_type, vnext(&mut sections), vnext(&mut sections))
                }
            },
        );
    }
    (raw_line, line, MarkdownSegments::Paragraph)
}

fn get_binding(sections: Split<&str>) -> String {
    sections.collect::<Vec<&str>>().join(" ")
}

fn get_section_type(sections: &mut Split<&str>) -> String {
    sections.next().unwrap_or("paper").to_lowercase()
}

fn vnext(sections: &mut Split<&str>) -> String {
    sections
        .next()
        .unwrap_or_default()
        .replace('\"', EMPTY_STRING)
}
