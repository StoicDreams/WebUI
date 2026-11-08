extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

const DEFAULT_OPTION_RETURN: &str = "()";

/// Extract the value of an option, or return from the current method if the option is None.
/// Default return on None is `()`.
/// Optionally pass desired return from None as second option.
/// Example:
/// ```rust,ignore
/// let data_option = Some("Hello");
/// let data = option!(data_option);
/// println!("{} World", data);
/// ```
#[proc_macro]
pub fn option(stream: TokenStream) -> TokenStream {
    // Tokens are kept as they are, so spaces inside strings and literal return values survive.
    let mut option = TokenStream::new();
    let mut missing_returns = TokenStream::new();
    let mut build_option = true;
    for token in stream {
        if !build_option {
            missing_returns.extend([token]);
            continue;
        }
        match &token {
            TokenTree::Punct(punct) if punct.as_char() == ',' => build_option = false,
            TokenTree::Literal(literal) => panic!("Unexpected literal:{}", literal),
            _ => option.extend([token]),
        }
    }
    if option.is_empty() {
        panic!("Missing Option variable");
    }
    let missing_returns = if missing_returns.is_empty() {
        DEFAULT_OPTION_RETURN.to_string()
    } else {
        missing_returns.to_string()
    };
    let result = format!(
        r#"match {}.to_owned() {{
    Some(value) => value,
    None => return {},
}}"#,
        option, missing_returns
    );
    result.parse().unwrap()
}

/// Simplify use of serde_json::to_string(&json!(...)).unwrap_or_default() to serialize data into a json string.
///
/// Example:
/// ```rust,ignore
/// let json = json_string!({
///     "name": "John",
///     "age": 47
/// });
/// ```
#[proc_macro]
pub fn json_string(stream: TokenStream) -> TokenStream {
    let mut body = String::default();
    stream.into_iter().for_each(|x| {
        if let TokenTree::Group(group) = x {
            body = group.to_string();
        }
    });
    if body.is_empty() {
        panic!("No data provided");
    }
    format!(
        r#"serde_json::to_string(&json!({})).unwrap_or_default()"#,
        body
    )
    .parse()
    .unwrap()
}
