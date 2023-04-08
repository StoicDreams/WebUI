use std::collections::HashMap;

pub(super) fn replace_tags(markdown: &str, tags: &HashMap<String, String>) -> String {
    let mut markdown = String::from(markdown);
    for tag in tags.keys() {
        if let Some(value) = tags.get(tag) {
            markdown = markdown.replace(&format!("{{{}}}", tag), value.as_str());
        }
    }
    markdown
}
