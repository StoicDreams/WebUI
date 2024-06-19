/* Template for web components. */
"use strict"
{
    // Start open Template
    class OpenTemplate extends HTMLElement {
        constructor() {
            super();
            const t = this;
        }
        static get observedAttributes() {
            return [];
        }
        attributeChangedCallback(property, oldValue, newValue) {
            if (oldValue === newValue) return;
            if (newValue === null || newValue === undefined) {
                delete this[property];
            } else {
                this[property] = newValue;
            }
        }
        connectedCallback() { }
        disconnectedCallback() { }
    }
    customElements.define('app-open-template', OpenTemplate);
    // End Open Template

    // Start Shadow Template
    const template = document.createElement('template')
    template.setAttribute('shadowrootmode', true);
    template.innerHTML = `
<style type="text/css">
:host {
}
</style>
<slot></slot>
<slot name="something"></slot>
`;
    class ShadowTemplate extends HTMLElement {
        constructor() {
            super();
            const t = this;
            const shadow = t.attachShadow({ mode: 'open' });
            t.template = template.content.cloneNode(true);
            shadow.appendChild(t.template);
        }
        static get observedAttributes() {
            return [];
        }
        attributeChangedCallback(property, oldValue, newValue) {
            if (oldValue === newValue) return;
            if (newValue === null || newValue === undefined) {
                delete this[property];
            } else {
                this[property] = newValue;
            }
        }
        connectedCallback() { }
        disconnectedCallback() { }
    }
    customElements.define('app-shadow-template', ShadowTemplate);
    // End Shadow Template
}
