use serde_json::json;
use webui_procs::{json_string, option};

fn double_or_unit(value: Option<i32>, out: &mut i32) {
    let value = option!(value);
    *out = value * 2;
}

fn double_or_none(value: Option<i32>) -> Option<i32> {
    let value = option!(value, None);
    Some(value * 2)
}

fn double_or_literal(value: Option<i32>) -> i32 {
    let value = option!(value, -1);
    value * 2
}

fn double_or_error(value: Option<i32>) -> Result<i32, String> {
    let value = option!(value, Err(String::from("no value given")));
    Ok(value * 2)
}

struct Holder {
    value: Option<String>,
}

fn field_or_empty(holder: &Holder) -> String {
    let value = option!(holder.value, String::new());
    value
}

#[test]
fn option_gives_the_value_when_there_is_one() {
    let mut out = 0;
    double_or_unit(Some(4), &mut out);
    assert_eq!(out, 8);
    assert_eq!(double_or_none(Some(4)), Some(8));
    assert_eq!(double_or_literal(Some(4)), 8);
    assert_eq!(double_or_error(Some(4)), Ok(8));
}

#[test]
fn option_returns_early_when_there_is_no_value() {
    let mut out = 7;
    double_or_unit(None, &mut out);
    assert_eq!(out, 7, "nothing after option! runs");
    assert_eq!(double_or_none(None), None);
}

#[test]
fn option_can_return_a_literal() {
    assert_eq!(double_or_literal(None), -1);
}

#[test]
fn option_keeps_spaces_inside_the_return_value() {
    assert_eq!(double_or_error(None), Err(String::from("no value given")));
}

#[test]
fn option_works_on_a_field_and_leaves_it_in_place() {
    let holder = Holder {
        value: Some(String::from("kept")),
    };
    assert_eq!(field_or_empty(&holder), "kept");
    assert_eq!(holder.value.as_deref(), Some("kept"));
    assert_eq!(field_or_empty(&Holder { value: None }), "");
}

#[test]
fn json_string_serializes_the_json() {
    let name = "John Smith";
    let text = json_string!({ "name": name, "age": 47, "tags": ["a b", "c"] });
    let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(
        parsed,
        json!({ "name": "John Smith", "age": 47, "tags": ["a b", "c"] })
    );
}
