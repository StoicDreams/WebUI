/* Template for open web components. */
"use strict"
{
    function toCamel(property) {
        return property.replace(/(-[A-Za-z0-9]{1})/g, a => { return a[1].toUpperCase(); });
    }
    class OpenTemplate extends HTMLElement {
        constructor() {
            super();
            const t = this;
            if (t.parentNode && t.parentNode.nodeName === 'P') {
                let p = t.parentNode;
                t.parentNode.parentNode.insertBefore(t, t.parentNode);
                if (p.innerHTML.trim() === '') {
                    p.remove();
                }
            }
        }
        static get observedAttributes() {
            return [];
        }
        attributeChangedCallback(property, oldValue, newValue) {
            property = toCamel(property);
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
}

/* Template for open web components. */
"use strict"
{
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
    function toCamel(property) {
        return property.replace(/(-[A-Za-z0-9]{1})/g, a => { return a[1].toUpperCase(); });
    }
    class ShadowTemplate extends HTMLElement {
        constructor() {
            super();
            const t = this;
            if (t.parentNode && t.parentNode.nodeName === 'P') {
                let p = t.parentNode;
                t.parentNode.parentNode.insertBefore(t, t.parentNode);
                if (p.innerHTML.trim() === '') {
                    p.remove();
                }
            }
            const shadow = t.attachShadow({ mode: 'open' });
            t.template = template.content.cloneNode(true);
            shadow.appendChild(t.template);
        }
        static get observedAttributes() {
            return [];
        }
        attributeChangedCallback(property, oldValue, newValue) {
            property = toCamel(property);
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
}
