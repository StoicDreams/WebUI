use super::*;

#[derive(Clone, Debug, PartialEq)]
pub(super) enum MarkdownSegments {
    AutoMaxContent(String, String),
    Cards(String, String),
    Card(String, u16, String, String, String, String),
    Code(String, String, String),
    Component,
    EndSection,
    Loading(LoadingVariant, Theme, u16, Option<u8>, String, String),
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
