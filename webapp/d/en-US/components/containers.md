<webui-data data-page-title="Container Components" data-page-subtitle=""></webui-data>

<webui-page-segment>

Web UI Container components are components that are expected to contain child components and/or other content passed through attributes.

</webui-page-segment>

### page-segment-standard

<webui-page-segment>

Use this component for standard page section padding.

</webui-page-segment>

<webui-side-by-side>

<webui-flex align="center" justify="center">

<webui-paper class="theme-white">

<webui-page-segment class="theme-info">

Example page-segment-standard

</webui-page-segment>

</webui-paper>

</webui-flex>

```html
<webui-flex align="center" justify="center">
    <webui-paper class="theme-white">
        <webui-page-segment class="theme-info">
            Example page-segment-standard
        </webui-page-segment>
    </webui-paper>
</webui-flex>
```

</webui-side-by-side>

### webui-cards

<webui-page-segment>

Use this component for card elements.

</webui-page-segment>

<webui-side-by-side>

<webui-cards>

<webui-card>

Example Card

</webui-card>

<webui-card name="Example">

Example Card

</webui-card>

<webui-card avatar="solid star">

Example Card

</webui-card>

<webui-card name="Example" avatar="solid star">

Example Card

</webui-card>

<webui-card name="Example" avatar="solid star" link="/">

Example Card

</webui-card>

<webui-card>

Example Card

</webui-card>

<webui-card>

Example Card

</webui-card>

<webui-card>

Example Card

</webui-card>

<webui-card>

Example Card

</webui-card>

<webui-card>

Example Card

</webui-card>

</webui-cards>

````html
<webui-cards>
    <webui-card>
        Example Card
    </webui-card>
    <webui-card name="Example">
        Example Card
    </webui-card>
    <webui-card avatar="solid star">
        Example Card
    </webui-card>
    <webui-card name="Example" avatar="solid star">
        Example Card
    </webui-card>
    <webui-card name="Example" avatar="solid star" link="/">
        Example Card
    </webui-card>
    ...
</webui-cards>
````

</webui-side-by-side>

### side-by-side

<webui-page-segment>

Use this class for a 2 column container.

</webui-page-segment>

<webui-side-by-side>

<webui-side-by-side>

<webui-paper elevation="10">

Example Paper

</webui-paper>

<webui-paper elevation="10">

Example Paper

</webui-paper>

</webui-side-by-side>

```html
<webui-side-by-side class="side-by-side gap-4">
    <webui-paper elevation="10">
        {"Example Paper"}
    </webui-paper>
    <webui-paper elevation="10">
        {"Example Paper"}
    </webui-paper>
</webui-side-by-side>
```

</webui-side-by-side>
