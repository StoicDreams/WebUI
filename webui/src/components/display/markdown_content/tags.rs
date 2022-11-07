use std::collections::HashMap;

pub(super) fn replace_tags(markdown: &str, tags: &HashMap<String, String>) -> String {
    let mut markdown = String::from(markdown);
    for tag in tags.keys() {
        match tags.get(tag) {
            Some(value) => {
                markdown = markdown.replace(&format!("{{{}}}", tag), value.as_str());
            }
            None => (),
        }
    }
    markdown
}
