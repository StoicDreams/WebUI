/* Template for Web UI components. */
"use strict"
webui.define("app-template", {
    constructor: (t) => { },
    attr: ['example'],
    attrChanged: (t, property, value) => {
        switch (property) {
            case 'example':
                break;
        }
    },
    connected: function (t) { },
    disconnected: function (t) { }
});

/* Template for Web UI components. */
"use strict"
webui.define("app-shadow-template", {
    constructor: (t) => { },
    attr: ['example'],
    attrChanged: (t, property, value) => {
        switch (property) {
            case 'example':
                break;
        }
    },
    connected: function (t) { },
    disconnected: function (t) { },
    shadowTemplate: `
<style type="text/css">
:host {
}
</style>
<slot></slot>
<slot name="something"></slot>
`
});
