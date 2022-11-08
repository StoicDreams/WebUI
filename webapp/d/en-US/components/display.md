# Display Components

```section
Web UI Display components are components that are expected to display some specific set of data or content.
```

## Avatar

```section
Avatar display components are used when you want to display either an image or icon (e.g. `fa-solid fa-acorn`) within a Paper.avatar container.

You can use an avatar in inline text, such as ![](/Logo.svg) or ![](fa-solid fa-acorn). If an avatar image is within a paragraph tag, then it will default its size to match the current font size within that paragraph.
```

````sidebyside

```paper
```sidebyside
```paper  "ma-10 pa-10"
![](/Logo.svg)
```
```paper  "ma-10 pa-10 f10 d-flex align-center"
![](fa-solid fa-acorn)
```
```paper  "ma-10 pa-10"
![Logo](/Logo.svg)
```
```paper  "ma-10 pa-10 f5 d-flex align-center"
![Acorn](fa-solid fa-acorn)
```
```
```

```rust
<Paper class={CLASSES_SIDE_BY_SIDE} elevation={ELEVATION_STANDARD}>
	<Paper class="ma-10 pa-10">
		<Avatar image="/Logo.svg" />
	</Paper>
	<Paper class="ma-10 pa-10 f10 d-flex align-center">
		<Avatar icon="fa-solid fa-acorn" />
	</Paper>
	<Paper class="ma-10 pa-10">
		<Avatar image="/Logo.svg" alt="Logo" />
	</Paper>
	<Paper class="ma-10 pa-10 f5 d-flex align-center">
		<Avatar icon="fa-solid fa-acorn" alt="Acorn" />
	</Paper>
</Paper>
```
````

## Image

```section

```

````sidebyside

```paper "ma-10 pa-10"
![Logo](/Logo.svg)
```
```rust
<Paper class="ma-10 pa-10">
	<Image src="/Logo.svg" alt="Logo" title="Logo" />
</Paper>
```
````

## Loading

```section
```

````sidebyside

```paper
```cards
>loading "circle" "primary" "32"
>loading "circle" "secondary" "32"
>loading "circle" "tertiary" "32"
>loading "circle" "success" "32"
>loading "circle" "info" "32"
>loading "circle" "warning" "32"
>loading "circle" "danger" "32"
>loading "circle" "active" "32"
>loading "circle" "black" "32"
>loading "circle" "white" "32"
>loading "circle" "title" "32"
>loading "circle" "shade" "32"
```
```cards
>loading "circle" "primary" "8"
>loading "circle" "primary" "16"
>loading "circle" "primary" "32"
>loading "circle" "primary" "64"
>loading "circle" "primary" "128"
```
```maxauto
LOADING_SIZE_TINY
>loading "bar" "primary" "8"
```
```maxauto
12u16
>loading "bar" "primary" "12"
```
```maxauto
LOADING_SIZE_SMALL
>loading "bar" "primary" "16"
```
```maxauto
LOADING_SIZE_MEDIUM
>loading "bar" "primary" "32"
```
```maxauto
LOADING_SIZE_LARGE
>loading "bar" "primary" "64"
```
```maxauto
LOADING_SIZE_XLARGE
>loading "bar" "primary" "128"
```
```maxauto
LOADING_SIZE_TINY
>loading "striped" "primary" "8"
```
```maxauto
12u16
>loading "striped" "primary" "12"
```
```maxauto
LOADING_SIZE_SMALL
>loading "striped" "primary" "16"
```
```maxauto
LOADING_SIZE_MEDIUM
>loading "striped" "primary" "32"
```
```maxauto
LOADING_SIZE_LARGE
>loading "striped" "primary" "64"
```
```maxauto
LOADING_SIZE_XLARGE
>loading "striped" "primary" "128"
```
```
```rust
<Cards>
	<Paper>
		<Loading variant={LoadingVariant::Circle} color={Theme::Primary} size={LOADING_SIZE_MEDIUM}>
	</Paper>
	<Paper>
		<Loading variant={LoadingVariant::Circle} color={Theme::Secondary} size={LOADING_SIZE_MEDIUM}>
	</Paper>
	...
</Cards>
<Cards>
	<Paper>
		<Loading variant={LoadingVariant::Circle} color={Theme::Primary} size={LOADING_SIZE_MEDIUM}>
	</Paper>
	<Paper>
		<Loading variant={LoadingVariant::Circle} color={Theme::Primary} size={LOADING_SIZE_LARGE}>
	</Paper>
	<Paper>
		<Loading variant={LoadingVariant::Circle} color={Theme::Primary} size={LOADING_SIZE_XLARGE}>
	</Paper>
</Cards>
<Paper class={CLASSES_MAXCONTENT_AUTO} elevation={ELEVATION_STANDARD}>
	<p>{"LOADING_SIZE_TINY"}</p>
	<Loading variant={LoadingVariant::Bar} color={Theme::Primary} size={LOADING_SIZE_TINY}>
</Paper>
<Paper class={CLASSES_MAXCONTENT_AUTO} elevation={ELEVATION_STANDARD}>
	<p>{"12u16"}</p>
	<Loading variant={LoadingVariant::Bar} color={Theme::Primary} size={12u16}>
</Paper>
...
<Paper class={CLASSES_MAXCONTENT_AUTO} elevation={ELEVATION_STANDARD}>
	<p>{"LOADING_SIZE_SMALL"}</p>
	<Loading variant={LoadingVariant::StripedBar} color={Theme::Primary} size={LOADING_SIZE_SMALL}>
</Paper>
...
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
			{
				Table::<workflow_details>::new(columns)
					.add_class("mt-3 mb-3".to_string())
					.bordered()
					.elevation(ELEVATION_STANDARD)
					.render(data)
			}
		</Paper>
	)
}
```

````
