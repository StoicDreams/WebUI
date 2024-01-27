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
    let mut option = String::default();
    let mut missing_returns = DEFAULT_OPTION_RETURN.to_string();
    let mut build_option = true;
    stream.into_iter().for_each(|x| match x {
        TokenTree::Ident(ident) => {
            if build_option {
                option = format!("{}{}", option, ident);
            } else {
                if missing_returns == DEFAULT_OPTION_RETURN {
                    missing_returns = String::default();
                }
                missing_returns = format!("{}{}", missing_returns, ident);
            }
        }
        TokenTree::Group(group) => {
            let group = group.to_string().replace(' ', "");
            if build_option {
                option = format!("{}{}", option, group);
            } else {
                missing_returns = format!("{}{}", missing_returns, group);
            }
        }
        TokenTree::Literal(literal) => {
            panic!("Unexpected literal:{}", literal);
        }
        TokenTree::Punct(punct) => {
            let punct = punct.to_string().replace(' ', "");
            if punct == "," || punct == " " {
                build_option = false;
            } else if build_option {
                option = format!("{}{}", option, punct);
            } else {
                missing_returns = format!("{}{}", missing_returns, punct);
            }
        }
    });
    if option.is_empty() {
        panic!("Missing Option variable");
    }
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
