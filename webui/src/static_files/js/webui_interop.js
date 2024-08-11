"use strict"

export function open_external_link(href, target) {
    let is_open_in_new_tab = target && target != '_self';
    if (is_open_in_new_tab) {
        if (window.__TAURI__) {
            window.__TAURI__.shell.open(href);
        } else {
            window.open(href, target);
        }
        return;
    }

    window.location.href = href;
}

export function show_alert(message, variant) {
    if (window.webuiAlert) {
        webuiAlert(message, variant);
    }
}

export function run_method(method, args) {
    if (typeof window[method] !== 'function') {
        return null;
    }
    if (!Array.isArray(args)) {
        args = [args];
    }
    return window[method](...args);
}

export function get_path() {
    return get_full_path().split('?')[0].split('#')[0];
}

export function get_full_path() {
    return location.href.substring(location.origin.length);
}

export function get_host() {
    return location.host;
}

export function get_origin() {
    return location.origin;
}

export function log(message) {
    console.log(message, TimeStamp());
}
function TimeStamp() {
    const now = Date.now();
    const milliseconds = Math.floor(now % 1000);
    const seconds = Math.floor((now / 1000) % 60);
    const minutes = Math.floor((now / 1000 / 60) % 60);
    const hours = Math.floor(now / 1000 / 60 / 60 % 24);

    return [
        pad(hours),
        pad(minutes),
        pad(seconds),
        pad(milliseconds)
    ].join(':');
    function pad(number) { return number < 10 ? `0${number}` : number; }
}

const GLOBALDATA = {};
export function set_global_data(key, data) {
    GLOBALDATA[key] = data;
}

export function get_global_data(key) {
    return GLOBALDATA[key] ?? '';
}

export function get_uuid() {
    try {
        return crypto.randomUUID();
    } catch (ex) {
        return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
            let r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
            return v.toString(16);
        });
    }
}

export async function webui_fetch(url, jsonIn, useCors) {
    let options = JSON.parse(jsonIn);
    if (useCors) {
        options.credentials = 'include';
        options.mode = 'cors';
        options.referrerPolicy = 'origin-when-cross-origin';
    }
    let result = await fetch(url, options);
    let headers = result.headers;
    let status = result.status;
    let body = await result.text();
    let json = JSON.stringify({
        headers: headers,
        status: status,
        body: body
    });
    return json;
}

function setupTauriIntegrations() {
    if (!window.__TAURI__) return;
    console.log('Setup Tauri integrations!');

    getEl('#titlebar-minimize', 30000).then(el => {
        el.addEventListener('click', () => window.__TAURI__.window.appWindow.minimize());
    });
    getEl('#titlebar-maximize', 30000).then(el => {
        el.addEventListener('click', async () => {
            if (await await window.__TAURI__.window.appWindow.isMaximized()) {
                el.innerHTML = el.getAttribute('data-maximize') || `<webui-icon icon="window-maximize" family="regular"></webui-icon>`;
            } else {
                el.innerHTML = el.getAttribute('data-restore') || `<webui-icon icon="window-restore" family="regular"></webui-icon>`;
            }
            window.__TAURI__.window.appWindow.toggleMaximize();
        })
    });
    getEl('#titlebar-close', 30000).then(el => {
        el.addEventListener('click', () => window.__TAURI__.window.appWindow.close());
    });
}

const getEl = function getEl(sel, mswait) {
    mswait = mswait || 0;
    return new Promise((resolve, _) => {
        let el = null;
        const start = Date.now();
        (function check() {
            el = document.querySelector(sel);
            if (el) {
                resolve(el);
                return;
            }
            const c = Date.now();
            if (c - start > mswait) {
                resolve(null);
                return;
            }
            setTimeout(check, 10);
        })();
    });
};

setTimeout(() => {
    setupTauriIntegrations();
}, 1000);
