# Themes

```section
Web UI groups various color settings into theme classes.

Each theme consists of a background color and foreground color (e.g. text), which is designated through css color variables paired in groups of color and color-offset.

Each theme applies the applicable color to the background with its offset value for the font (foreground) color.
```

## Theme Classes

```section
Theme classes are helpers for setting applicable background and foreground colors to an element.
```

### theme-primary

```section
This theme applies the primary color - css variables `--color-primary` and `--color-primary-offset` for background and foreground respectively.
```

````sidebyside
```paper "theme-primary elevation-10 ma-5 pa-5"
Example class theme-primary
```
```rust
<Paper class="theme-primary ma-5 pa-5" elevation={ELEVATION_STANDARD}>
	{"Example class theme-primary"}
</Paper>
```
````

### theme-secondary

```section
This theme applies the secondary color - css variables `--color-secondary` and `--color-secondary-offset` for background and foreground respectively.
```

````sidebyside
```paper "theme-secondary elevation-10 ma-5 pa-5"
Example class theme-secondary
```
```rust
<Paper class="theme-secondary ma-5 pa-5" elevation={ELEVATION_STANDARD}>
	{"Example class theme-secondary"}
</Paper>
```
````

### theme-tertiary

```section
This theme applies the tertiary color - css variables `--color-tertiary` and `--color-tertiary-offset` for background and foreground respectively.
```

````sidebyside
```paper "theme-tertiary elevation-10 ma-5 pa-5"
Example class theme-tertiary
```
```rust
<Paper class="theme-tertiary ma-5 pa-5" elevation={ELEVATION_STANDARD}>
	{"Example class theme-tertiary"}
</Paper>
```
````

### theme-success

```section
This theme applies the success color - css variables `--color-success` and `--color-success-offset` for background and foreground respectively.
```

````sidebyside
```paper "theme-success elevation-10 ma-5 pa-5"
Example class theme-success
```
```rust
<Paper class="theme-success ma-5 pa-5" elevation={ELEVATION_STANDARD}>
	{"Example class theme-success"}
</Paper>
```
````

### theme-info

```section
This theme applies the info color - css variables `--color-info` and `--color-info-offset` for background and foreground respectively.
```

````sidebyside
```paper "theme-info elevation-10 ma-5 pa-5"
Example class theme-info
```
```rust
<Paper class="theme-info ma-5 pa-5" elevation={ELEVATION_STANDARD}>
	{"Example class theme-info"}
</Paper>
```
````

### theme-warning

```section
This theme applies the warning color - css variables `--color-warning` and `--color-warning-offset` for background and foreground respectively.
```

````sidebyside
```paper "theme-warning elevation-10 ma-5 pa-5"
Example class theme-warning
```
```rust
<Paper class="theme-warning ma-5 pa-5" elevation={ELEVATION_STANDARD}>
	{"Example class theme-warning"}
</Paper>
```
````

### theme-danger

```section
This theme applies the danger color - css variables `--color-danger` and `--color-danger-offset` for background and foreground respectively.
```

````sidebyside
```paper "theme-danger elevation-10 ma-5 pa-5"
Example class theme-danger
```
```rust
<Paper class="theme-danger ma-5 pa-5" elevation={ELEVATION_STANDARD}>
	{"Example class theme-danger"}
</Paper>
```
````

### theme-title

```section
This theme applies the title color - css variables `--color-title` and `--color-title-offset` for background and foreground respectively.
```

````sidebyside
```paper "theme-title elevation-10 ma-5 pa-5"
Example class theme-title
```
```rust
<Paper class="theme-title ma-5 pa-5" elevation={ELEVATION_STANDARD}>
	{"Example class theme-title"}
</Paper>
```
````

### theme-inherit

```section
This theme applies inheritance to the background and foreground color.
```

````sidebyside
```paper "theme-inherit elevation-10 ma-5 pa-5"
Example class theme-inherit
```
```rust
<Paper class="theme-inherit ma-5 pa-5" elevation={ELEVATION_STANDARD}>
	{"Example class theme-inherit"}
</Paper>
```
````

### theme-active

```section
This theme applies the active color - css variables `--color-active` and `--color-active-offset` for background and foreground respectively.
```

````sidebyside
```paper "theme-active elevation-10 ma-5 pa-5"
Example class theme-active
```
```rust
<Paper class="theme-active ma-5 pa-5" elevation={ELEVATION_STANDARD}>
	{"Example class theme-active"}
</Paper>
```
````

### theme-background

```section
This theme applies the background color - css variables `--color-background` and `--color-background-offset` for background and foreground respectively.
```

````sidebyside
```paper "theme-background elevation-10 ma-5 pa-5"
Example class theme-background
```
```rust
<Paper class="theme-background ma-5 pa-5" elevation={ELEVATION_STANDARD}>
	{"Example class theme-background"}
</Paper>
```
````

### theme-black

```section
This theme applies the black color - css variables `--color-black` and `--color-black-offset` for background and foreground respectively.
```

````sidebyside
```paper "theme-black elevation-10 ma-5 pa-5"
Example class theme-black
```
```rust
<Paper class="theme-black ma-5 pa-5" elevation={ELEVATION_STANDARD}>
	{"Example class theme-black"}
</Paper>
```
````

### theme-white

```section
This theme applies the white color - css variables `--color-white` and `--color-white-offset` for background and foreground respectively.
```

````sidebyside
```paper "theme-white elevation-10 ma-5 pa-5"
Example class theme-white
```
```rust
<Paper class="theme-white ma-5 pa-5" elevation={ELEVATION_STANDARD}>
	{"Example class theme-white"}
</Paper>
```
````

## Color Settings

````sidebyside
```paper
```section
Set your own color values for your Web UI app by adding and customizing this css code snippet to your site's css file.
```
```quote "info" "" "mt-3"
**Note:** You should not update the webui.css file that contains the default settings, unless you plan to never run the `webui` auto updater that applies the latest updates.
```
```
```css
:root {
	--color-black: #101010FF;
	--color-white: #FFFFFFFF;
	--site-background-color: #333333FF;
	--site-background-offset: var(--color-white);
	--color-title: #b13b3bFF;
	--color-title-offset: var(--color-white);
	--color-primary: #9e091aFF;
	--color-primary-offset: var(--color-white);
	--color-secondary: #2e54ffFF;
	--color-secondary-offset: var(--color-white);
	--color-tertiary: #7d0b77FF;
	--color-tertiary-offset: var(--color-white);
	--color-info: #2196f3FF;
	--color-info-offset: var(--color-white);
	--color-success: #1c970eFF;
	--color-success-offset: var(--color-white);
	--color-warning: rgb(191, 84, 31);
	--color-warning-offset: var(--color-white);
	--color-danger: rgb(183, 5, 5);
	--color-danger-offset: var(--color-white);
	--color-shade: rgba(125,125,125,0.2);
	--color-active: var(--color-info);
	--color-active-offset: var(--color-info-offset);
	--color-button: var(--color-info);
	--color-button-offset: var(--color-info-offset);
}
```
````
