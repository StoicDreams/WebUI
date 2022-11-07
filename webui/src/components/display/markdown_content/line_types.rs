use std::str::Split;
use super::*;

pub(super) fn get_line_type(line: &str) -> (String, MarkdownSegments) {
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
        while line.starts_with("`") {
            line.replace_range(0..1, "");
        }
        if line.is_empty() {
            return (line, MarkdownSegments::EndSection);
        }
        let mut sections = line.split(" ");
        let section_type = sections.next().unwrap_or("paper").to_lowercase();
        let sections = sections.to_owned().collect::<Vec<&str>>();
        let line = sections.join(" ");
        let mut sections = line.split("\" \"").to_owned();
        let mut test = sections.to_owned();
        return (
            section_type.to_string(),
            match section_type.as_str() {
                "cards" => MarkdownSegments::Cards(next(&mut sections), next(&mut sections)),
                "card" => MarkdownSegments::Card(
                    next(&mut sections),
                    u16::from_str(&next(&mut sections)).unwrap_or(800u16),
                    next(&mut sections),
                    next(&mut sections),
                    next(&mut sections),
                ),
                "list" => MarkdownSegments::List(
                    !next(&mut sections).is_empty(),
                    next(&mut sections),
                    next(&mut sections),
                ),
                "quote" => MarkdownSegments::Quote(
                    next(&mut sections),
                    next(&mut sections),
                    next(&mut sections),
                    next(&mut sections),
                ),
                "section" => {
                    MarkdownSegments::PageSection(next(&mut sections), next(&mut sections))
                }
                "sidebyside" => {
                    MarkdownSegments::SideBySide(next(&mut sections), next(&mut sections))
                }
                "sideimage" => MarkdownSegments::SideImage(
                    next(&mut sections),
                    next(&mut sections),
                    next(&mut sections),
                    next(&mut sections),
                ),
                "paper" => MarkdownSegments::Paper(next(&mut sections), next(&mut sections)),
                _ => MarkdownSegments::Code(section_type, next(&mut sections), next(&mut sections)),
            },
        );
    }
    (line, MarkdownSegments::Paragraph)
}

fn next(sections: &mut Split<&str>) -> String {
    sections
        .next()
        .unwrap_or_default()
        .replace("\"", "")
        .to_string()
}
