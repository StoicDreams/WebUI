use crate::prelude::*;

/// general handler to replace keys wrapped with a deliminator key or keys.
pub fn replace_tag_markers(text: &str, delim: &str, handler: fn(&str) -> Option<String>) -> String {
    let mut result = String::default();
    let segments = text.split(delim).collect::<Vec<&str>>();
    let count = segments.len();
    if count == 2 {
        return text.to_owned();
    }
    let mut index = 0;
    let mut prefix = "";
    for segment in segments {
        index += 1;
        if index == count {
            result = format!("{}{}{}", result, prefix, segment);
            break;
        }
        if prefix == "" {
            result = format!("{}{}", result, segment);
            prefix = delim;
            continue;
        }
        match handler(segment) {
            Some(emoji) => {
                result = format!("{}{}", result, emoji);
                prefix = "";
            }
            None => {
                result = format!("{}{}{}", result, prefix, segment);
                prefix = delim;
            }
        }
    }
    result.to_owned()
}

/// General handler to replace keys within text that are placeholders for other start and end tags
pub fn replace_start_end_deliminators(text: &str, delim: &str, start: &str, end: &str) -> String {
    let mut result = String::default();
    let segments = text.split(delim).collect::<Vec<&str>>();
    let count = segments.len();
    if count == 2 {
        return text.to_owned();
    }
    let mut index = 0;
    let mut skip = true;
    for segment in segments {
        index += 1;
        if index == count {
            result = format!("{}{}", result, segment);
            break;
        }
        if skip {
            skip = false;
            result = format!("{}{}", result, segment);
            continue;
        }
        result = format!("{}{}{}{}", result, start, segment, end);
        skip = true;
    }
    result.to_owned()
}

/// Trim any left-side whitespace that is the minimum whitespace length covering all lines.
/// 
/// Example
/// ```rust
/// use webui::prelude::*;
/// 
/// fn main() {
///     let test = r#"
///         Line 1
///             Line 2
///                 Line 3
///         line 4
///         Line 5
///         "#;
///     let result = trim_left_padding(test);
///     let expect = "Line 1\n    Line 2\n        Line 3\nline 4\nLine 5";
///     assert_eq!(expect, result.trim());
/// }
/// ```
pub fn trim_left_padding(text: &str) -> String {
    let min_spaces = text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);
    if min_spaces == 0 { return text.to_string(); }
    text.lines()
        .map(|line| {
            if line.is_empty() { return line.trim(); }
            &line[min_spaces..]
        })
        .collect::<Vec<_>>()
        .join("\n")
}
