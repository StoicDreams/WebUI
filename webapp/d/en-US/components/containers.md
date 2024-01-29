# Container Components

```section
Web UI Container components are components that are expected to contain child components.
```

## Paper

```section
Paper components are the base container for which to hold other elements or data in.
```

````sidebyside
```paper
Your content goes here
```
```rust
<Paper>{"Your content goes here"}</Paper>
```
````

## Card

```section
Card components are special containers that include a header, body, and footer, with the header optionally containing Avatar, Title, and Link segments.
```

`````sidebyside
````cards
```card "Hello" "300"
World
```
```card "Foo" "300"
Foo
```
```card "Lorem" "400" "info" "fa-solid fa-acorn" "https://loremipsum.io/"
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
```
````
````rust
<Cards>
    <Card title="Hello" width={300}>
        {"World"}
    </Card>
    <Card title="Foo" width={300}>
        {"Bar"}
    </Card>
    <Card title="Lorem" width={400} color={Theme::Info} avatar="fa-solid fa-acorn" link="https://loremipsum.io/">
        {"Lorem ipsum dolor..."}
    </Card>
</Cards>
````
`````

## List

```section
List components are containers for displaying lists.
```

`````sidebyside
````sidebyside
```list
One
Two
Three
```
```list "true" "pa-2 pl-6 elevation-10"
One
Two
Three
```
````
````rust
<Paper class={classes!(CLASSES_SIDE_BY_SIDE, "gap-2")} elevation={ELEVATION_STANDARD}>
    <List>
        {list_items!(
            "One",
            "Two",
            "Three",
        )}
    </List>
    <List is_ordered={true} class="pa-2 pl-6 elevation={ELEVATION_STANDARD}>
        {list_items!(
            "One",
            "Two",
            "Three",
        )}
    </List>
</Paper>
````
`````

## Markdown

```section
Markdown components allow loading content from markdown files. This is a great way to reduce the initial loading of the website, as markdown can be loaded dynamically from API data or static files.
```

````sidebyside
```Paper
# Title
!<Test>
```
```rust
<MarkdownContent markdown={r#"
    # Title
    !<Test>
    "#} />

<MarkdownContent href="/d/en-US/test.md />
```
````

## Quote

```section
Quote components are containers for displaying specialy highlighted text, as well as an optional citation if desired.
```

`````sidebyside
````paper
```quote
To cite or not to cite?
```
```quote "success" "Albert Einstein"
I have no special talent. I am only passionately curious.
```
```quote "warning" "Mark Twain"
Whenever you find yourself on the side of the majority, it is time to pause and reflect.
```
````

````rust
<Paper class={classes!(CLASSES_SIDE_BY_SIDE, "gap-2")} elevation={ELEVATION_STANDARD}>
    <Quote>
        {"To cite or not to cite?"}
    </Quote>
    <Quote cite="Albert Einstein" color={Theme::Success}>
        {"I have no special talent. I am only passionately curious."}
    </Quote>
    <Quote cite="Mark Twain" color={Theme::Warning}>
        {"Whenever you find yourself on the side of the majority, it is time to pause and reflect."}
    </Quote>
</Paper>
````
`````

## SideImage

```section
SideImage components are containers for displaying a standard side-by-side section, with either the left or right side designated with an image.
```

`````sidebyside
````paper
```sideimage "right" "/Logo.svg"
Image on the right.
```
```sideimage "left" "/Logo.svg"
Image on the left.
```
````
````rust
<SideImage image_pos={LeftOrRight::Right} src="/Logo.svg">
    {"Image on the right."}
</SideImage>
<SideImage image_pos={LeftOrRight::Left} src="/Logo.svg">
    {"Image on the left."}
</SideImage>
````
`````
