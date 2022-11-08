use super::*;

#[derive(Clone, Debug)]
pub(super) enum MarkdownSegments {
    AutoMaxContent(String, String),
    Cards(String, String),
    Card(String, u16, String, String, String),
    Code(String, String, String),
    EndSection,
    Loading(LoadingVariant, Theme, u16, String, String),
    List(bool, String, String),
    MaxContentAuto(String, String),
    PageSection(String, String),
    Paper(String, String),
    Quote(String, String, String, String),
    Paragraph,
    SideBySide(String, String),
    SideImage(String, String, String, String),
    Table(Vec<String>, String, String),
    Title(u8),
}
