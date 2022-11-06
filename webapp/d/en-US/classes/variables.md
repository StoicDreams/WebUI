# Rust Class Variable Helpers

```section

```

## Rust Constants

````section
These rust constant variables are available to access common/recommended configurations.

**Example Usage**:
```rust
use webui::prelude::*;

pub(crate) fn hello_world() -> Html {
	html!(
		<Paper class={CLASSES_PAGE_SECTION}>
			{"Hello World"}
		</Paper>
	)
}
```

````

## Class Helpers and Groupings

```section
The `CLASSES_*` helper variables contain class values for one or more classes. In most cases, a variable is only referencing a single class. But some variables will contain multiple classes.

See the [classes.rs file on GitHub](https://github.com/StoicDreams/RustWebUI/blob/main/webui/src/common/classes.rs) for the full suite of `CLASSES_*` variables.
```

### CLASSES_PAGE_SECTION

```section
This variable holds the class `page-segment-standard`, which applies a default margin and padding to the element.
```

````sidebyside
```paper "page-segment-standard elevation-10"
Example CLASSES_PAGE_SECTION
```

```rust
<Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
	{"Example CLASSES_PAGE_SECTION"}
</Paper>
```
````

### CLASSES_CARD_CONTAINER

```section
This variable holds the class `page-segment-cards`, which applies default css styles for a flex container meant for displaying card elements.
```

````sidebyside
```paper "page-segment-cards elevation-10"
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```paper "pa-2 elevation-10"
Example Card
```
```

```rust
<Paper class={CLASSES_CARD_CONTAINER} elevation={ELEVATION_STANDARD}>
	<Paper class="pa-2 elevation={ELEVATION_STANDARD>
		{"Example Card"}
	</Paper>
	<Paper class="pa-2 elevation={ELEVATION_STANDARD>
		{"Example Card"}
	</Paper>
	...
</Paper>
```
````

### CLASSES_SIDE_BY_SIDE

```section
This variable holds the class `side-by-side`, which applies default css styles for a grid container 
with 2 columns.
```

````sidebyside

```sidebyside
```paper "ma-5 pa-5 elevation-10"
Example Left
```

```paper "ma-5 pa-5 elevation-10"
Example Right
```
```

```paper
```rust
<Paper class={CLASSES_SIDE_BY_SIDE} elevation={ELEVATION_STANDARD}>
	<Paper class="ma-5 pa-5" elevation={ELEVATION_STANDARD}>
		{"Example Left"}
	</Paper>

	<Paper class="ma-5 pa-5" elevation={ELEVATION_STANDARD}>
		{"Example Right"}
	</Paper>
</Paper>
```
```
````

## Elevation Helpers

```section
Elevation variables contain u8 values that reflext the range of possible elevations that Web UI elements and styles support.

Elevations simply apply a box shadow around an element to produce an effect that the element is raised above its parent element when the elevation is greater than 0, up to the max elevation of 25.
```

### ELEVATION_ZERO

```section
This is equivelent to no elevation, which is the default for Paper elements.

Some other components may default to a higher elevation, in which case you can pass it this value to overwrite that default value to 0.
```

````sidebyside

```paper
```paper "ma-5 pa-5 elevation-0"
Example ELEVATION_ZERO
```

```paper "ma-5 pa-5"
Example No Elevation (default)
```
```

```paper
```rust
<Paper class="ma-5 pa-5" elevation={ELEVATION_ZERO}>
	{"Example ELEVATION_ZERO"}
</Paper>

<Paper class="ma-5 pa-5">
	{"Example No Elevation (default)"}
</Paper>
```
```
````

### ELEVATION_MIN

```section
This variable holds the u8 value of 1, which is the lowest elevation that displays a box shadow.
```

````sidebyside

```paper "ma-5 pa-5 elevation-1"
Example ELEVATION_MIN
```

```rust
<Paper class="ma-5 pa-5" elevation={ELEVATION_MIN}>
	{"Example ELEVATION_MIN"}
</Paper>
```

````

### ELEVATION_STANDARD

```section
This variable holds the u8 value of 10, which is the recommended standard elevation for most components that you want elevated.
```

````sidebyside

```paper "ma-5 pa-5 elevation-10"
Example ELEVATION_STANDARD
```

```rust
<Paper class="ma-5 pa-5" elevation={ELEVATION_STANDARD}>
	{"Example ELEVATION_STANDARD"}
</Paper>
```

````

### ELEVATION_MAX

```section
This variable holds the u8 value of 25, which is the maximum elevation value supported by Web UI.
```

````sidebyside

```paper "ma-5 pa-5 elevation-25"
Example ELEVATION_MAX
```

```rust
<Paper class="ma-5 pa-5" elevation={ELEVATION_MAX}>
	{"Example ELEVATION_MAX"}
</Paper>
```

````
