# Display Components

```section
Web UI Display components are components that are expected to display some specific set of data or content.
```

## Avatar

```section

```

````sidebyside

```paper
```
```rust
```

````

## Image

```section

```

````sidebyside

```paper
```
```rust
```

````

## Loading

```section

```

````sidebyside

```paper
```
```rust
```

````

## MarkdownContent

```section

```

````sidebyside

```paper
```
```rust
```

````

## Table

```section

```

````sidebyside

```paper
| One | Two |
| --- | --- |
| Hello | World|
| Foo | Bar |
| Lorem | Ipsum |
```
```rust
#[derive(PartialEq)]
struct table_detail {
	pub one: String,
	pub two: String,
}
fn example() -> Html {
	let columns = vec![
		TableColumns::<workflow_details>::new(
            "One".to_string(),
            |data| html! {data.one.to_string()},
        ),
		TableColumns::<workflow_details>::new(
            "Two".to_string(),
            |data| html! {data.one.to_string()},
        ),
	];
	let data = vec![
		table_detail {
			one: String::from("Hello"),
			two: String::from("World")
		},
		table_detail {
			one: String::from("Foo"),
			two: String::from("Bar")
		},
	];
	html!(
		<Paper>
			{Table::<workflow_details>::new(columns).add_class("mt-3 mb-3".to_string()).bordered().elevation(ELEVATION_STANDARD).render(data)}
		</Paper>
	)
}
```

````
