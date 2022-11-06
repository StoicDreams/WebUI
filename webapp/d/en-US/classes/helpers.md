# Web UI Helper Classes

```section
This page details the available helper classes, which are classes you can set for an element/component to achieve a desired affect associated with that class.
```

## Classes

### page-segment-standard

```section
Use this class to set default margin and padding for an element.
```

````sidebyside
```paper "d-flex align-center justify-center"
```paper "theme-white"
```paper "page-segment-standard theme-info"
Example page-segment-standard
```
```
```
```rust
<Paper class="d-flex align-center justify-center">
	<Paper class="theme-white">
		<Paper class="page-segment-standard theme-info">
			{"Example page-segment-standard"}
		</Paper>
	</Paper>
</Paper>
```
````

### page-segment-cards

```section
Use this class to a container for card elements.
```

````sidebyside
```paper "page-segment-cards"
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```paper "elevation-10 pa-3"
Example Card
```
```
```rust
<Paper class="page-segment-cards">
	<Paper class="elevation-10 pa-3">
		{"Example Card"}
	</Paper>
	<Paper class="elevation-10 pa-3">
		{"Example Card"}
	</Paper>
	...
</Paper>
```
````

### side-by-side

```section
Use this class to a container for card elements.
```

````sidebyside
```paper "side-by-side gap-4"
```paper "elevation-10 pa-3"
Example Paper
```
```paper "elevation-10 pa-3"
Example Paper
```
```
```rust
<Paper class="side-by-side gap-4">
	<Paper class="elevation-10 pa-3">
		{"Example Paper"}
	</Paper>
	<Paper class="elevation-10 pa-3">
		{"Example Paper"}
	</Paper>
</Paper>
```
````

### auto-maxcontent

```section
Use this class to a container for card elements.
```

````sidebyside
```paper "auto-maxcontent gap-4"
```paper "elevation-10 pa-3"
Example Paper
```
```paper "elevation-10 pa-3"
Example Paper
```
```
```rust
<Paper class="auto-maxcontent gap-4">
	<Paper class="elevation-10 pa-3">
		{"Example Paper"}
	</Paper>
	<Paper class="elevation-10 pa-3">
		{"Example Paper"}
	</Paper>
</Paper>
```
````

### maxcontent-auto

```section
Use this class to a container for card elements.
```

````sidebyside
```paper "maxcontent-auto gap-4"
```paper "elevation-10 pa-3"
Example Paper
```
```paper "elevation-10 pa-3"
Example Paper
```
```
```rust
<Paper class="maxcontent-auto gap-4">
	<Paper class="elevation-10 pa-3">
		{"Example Paper"}
	</Paper>
	<Paper class="elevation-10 pa-3">
		{"Example Paper"}
	</Paper>
</Paper>
```
````

### Font Adjustments

```section
These helper classes allow adjusting the font size
```

`````sidebyside
````cards
```paper "f0 elevation-10"
Example f0
```
```paper "f1 elevation-10"
Example f1
```
```paper "f2 elevation-10"
Example f2
```
```paper "f3 elevation-10"
Example f3
```
```paper "f4 elevation-10"
Example f4
```
```paper "f5 elevation-10"
Example f5
```
```paper "f6 elevation-10"
Example f6
```
```paper "f7 elevation-10"
Example f7
```
```paper "f8 elevation-10"
Example f8
```
```paper "f9 elevation-10"
Example f9
```
```paper "f10 elevation-10"
Example f10
```
````

````rust
<Paper class="page-section-cards">
	<Paper class="f0" elevation={ELEVATION_STANDARD}>
		{"Example f0}
	</Paper>
	<Paper class="f1" elevation={ELEVATION_STANDARD}>
		{"Example f1}
	</Paper>
	...
</Paper>
````
`````

### Margins & Padding

````sidebyside
```paper "ma-3 pa-3 elevation-10"
Margin and padding helper classes allow setting values that represent multipliers of the css variable `--padding`.

Acceptable multipler values are between 0 and 10, with zero being no margin or padding, and 10 being the max multipler value.

Use `ma-N` and `pa-N` margin and padding variants to set margin and padding for all 4 sides (top, right, bottom, and left).
```

```rust
<Paper class="ma-3 pa-3" elevation={ELEVATION_STANDARD}>
	{paragraphs!("Margin and padding helper...")}
</Paper>
```
````

````sidebyside
```paper "ml-1 mr-10 mt-5 mb-4 pa-1 pl-5 elevation-10"
You can also explicitely set any side by using the applicable side variant for top, right, bottom, and left - mt-N, mr-N, mb-N, ml-N, pt-N, pr-N, etc.
```

```rust
<Paper class="ml-1 mr-10 mt-5 mb-4 pa-1 pl-5" elevation={ELEVATION_STANDARD}>
	{paragraphs!("You can also explicitely...")}
</Paper>
```
````


````sidebyside
```paper "ma-3 mt-n1 pa-3 elevation-10"
Margin types also allow setting negative values.

Negative values can be set for any margin variant by simply prefixing an `n` character before the numeric digit of the class - class `mt-1` translates to style `margin-top: var(--padding)`, and class `mt-n1` translates to style `margin-top: calc(-1 * var(--padding))`.
```

```rust
<Paper class="ma-3 mt-n1 pa-3" elevation={ELEVATION_STANDARD}>
	{paragraphs!("Margin types also allow...")}
</Paper>
```
````
