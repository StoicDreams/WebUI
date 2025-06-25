<webui-data data-page-title="Container Components" data-page-subtitle=""></webui-data>

<webui-page-segment elevation="10">
    Web UI Container components are components that are expected to contain child components and/or other content passed through attributes.
</webui-page-segment>

### page-segment-standard

<webui-page-segment elevation="10">
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
    <webui-code lang="html">
        <webui-flex align="center" justify="center">
            <webui-paper class="theme-white">
                <webui-page-segment class="theme-info">
                    Example page-segment-standard
                </webui-page-segment>
            </webui-paper>
        </webui-flex>
    </webui-code>
</webui-side-by-side>

### webui-cards

<webui-page-segment elevation="10">
    Use this component for card elements.
</webui-page-segment>

<webui-side-by-side>
    <webui-cards card-width="300">
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
    <webui-code lang="html">
        <webui-cards card-width="220">
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
    </webui-code>
</webui-side-by-side>

### side-by-side

<webui-page-segment elevation="10">
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
    <webui-code lang="html">
        <webui-side-by-side class="side-by-side gap-4">
            <webui-paper elevation="10">
                {"Example Paper"}
            </webui-paper>
            <webui-paper elevation="10">
                {"Example Paper"}
            </webui-paper>
        </webui-side-by-side>
    </webui-code>
</webui-side-by-side>
