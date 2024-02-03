"use strict"

let app = null;
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

export function run_method(method, args) {
    if (typeof window[method] !== 'function') {
        return null;
    }
    if (!Array.isArray(args)) {
        args = [args];
    }
    return window[method](...args);
}

export function set_title(title) {
    document.title = title;
}

let pageTransitionDuration = 300;
export function push_state(path) {
    history.pushState(null, null, path);
}

export function set_page_transition_duration(value) {
    let number = parseInt(value);
    if (isNaN(number)) { return; }
    if (number < 0) { return; }
    if (number > 1000) { return; }
    pageTransitionDuration = number;
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

const STORAGE_ACCEPTED_KEY = 'storage_accepted';
const REJECT_STORAGE_CACHING = 0;
const ACCEPT_SESSION_STORAGE = 1;
const ACCEPT_LOCAL_STORAGE = 2;
const memStorage = (function () {
    const memStorageCache = {}
    let acceptedStorage = REJECT_STORAGE_CACHING;
    if (localStorage.key(STORAGE_ACCEPTED_KEY)) {
        acceptedStorage = ACCEPT_LOCAL_STORAGE;
        Object.keys(localStorage).forEach(key => {
            memStorageCache[key] = localStorage.getItem(key);
        });
    } else if (sessionStorage.key(STORAGE_ACCEPTED_KEY)) {
        acceptedStorage = ACCEPT_SESSION_STORAGE;
        Object.keys(sessionStorage).forEach(key => {
            memStorageCache[key] = sessionStorage.getItem(key);
        });
    }
    function getCache() {
        return new Promise((resolve, reject) => {
            if (localStorage.getItem(STORAGE_ACCEPTED_KEY)) {
                resolve(localStorage);
            } else if (sessionStorage.getItem(STORAGE_ACCEPTED_KEY)) {
                resolve(sessionStorage);
            } else {
                reject('Caching not accepted');
            }
        });
    }
    class MemStorage {
        key(key) {
            return Object.keys(memStorageCache).filter(m => m == key).length > 0 ? key : null;
        }
        setItem(key, value) {
            memStorageCache[key] = value;
            getCache(key).then(cache => {
                cache.setItem(key, value);
            });
        }
        getItem(key) {
            return memStorageCache[key] ?? "";
        }
        acceptLocalStorage() {
            acceptedStorage = ACCEPT_LOCAL_STORAGE;
            this.setItem(STORAGE_ACCEPTED_KEY, acceptedStorage);
            sessionStorage.clear();
            Object.keys(memStorageCache).forEach(key => {
                localStorage.setItem(key, memStorageCache[key]);
            });
        }
        acceptSessionStorage() {
            acceptedStorage = ACCEPT_SESSION_STORAGE;
            this.setItem(STORAGE_ACCEPTED_KEY, acceptedStorage);
            localStorage.clear();
            Object.keys(memStorageCache).forEach(key => {
                sessionStorage.setItem(key, memStorageCache[key]);
            });
        }
        rejectCachedStorage() {
            acceptedStorage = REJECT_STORAGE_CACHING;
            this.setItem(STORAGE_ACCEPTED_KEY, acceptedStorage);
            sessionStorage.clear();
            localStorage.clear();
        }
    }
    return new MemStorage();
})();

export function user_accepts_local_storage() {
    memStorage.acceptLocalStorage();
}

export function user_accepts_session_storage() {
    memStorage.acceptSessionStorage();
}

export function user_rejects_cached_storage() {
    memStorage.rejectCachedStorage();
}

export function set_user_storage_data(key, data) {
    memStorage.setItem(key, data);
}

export function get_user_storage_data(key) {
    return memStorage.getItem(key);
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

const APP_CLASS_KEY = "app_classes";
export function app_has_classes(classes) {
    classes = classes.split(' ');
    let current = app.className.split(' ');
    for(let i = 0;i < classes.length; ++i) {
        if (current.indexOf(classes[i]) !== -1) continue;
        return false;
    }
    return true;
}

export function update_app_classes(add_classes, remove_classes) {
    add_app_class(add_classes);
    remove_app_class(remove_classes);
}

export function add_app_class(classes) {
    let current = app.className.split(' ');
    classes.split(' ').forEach(cls => {
        if (!cls) return;
        if (current.indexOf(cls) != -1) return;
        current.push(cls);
    });
    app.className = current.join(' ');
    memStorage.setItem(APP_CLASS_KEY, app.className);
    applyDynamicStyleRules();
}

export function remove_app_class(classes) {
    let current = app.className.split(' ');
    let updated = [];
    classes = classes.split(' ');
    current.forEach(cls => {
        if (!cls) return;
        if (classes.indexOf(cls) != -1) return;
        updated.push(cls);
    });
    app.className = updated.join(' ');
    memStorage.setItem(APP_CLASS_KEY, app.className);
    applyDynamicStyleRules();
}

function loadAppClasses(){
    app.className = memStorage.getItem(APP_CLASS_KEY);
}

function setupTauriIntegrations(){
    if (!window.__TAURI__) return;
    console.log('Setup Tauri integrations!');

    getEl('#titlebar-minimize', 30000).then(el => {
        el.addEventListener('click', () => window.__TAURI__.window.appWindow.minimize());
    });
    getEl('#titlebar-maximize', 30000).then(el => {
        el.addEventListener('click', async () => {
            if (await await window.__TAURI__.window.appWindow.isMaximized()) {
                el.innerHTML = el.getAttribute('data-maximize') || `<i class="far fa-window-maximize"></i>`;
            } else {
                el.innerHTML = el.getAttribute('data-restore') || `<i class="far fa-window-restore"></i>`;
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

const set_body_class = function set_body_class(winWidth) {
    // Flag general width by class
    let w = winWidth > 3800 ? 'w-4k'
        : winWidth > 3400 ? 'w-wqhd'
            : winWidth > 2500 ? 'w-qhd'
                : winWidth > 1900 ? 'w-fhd'
                    : winWidth > 1500 ? 'w-hdp'
                        : winWidth > 1300 ? 'w-hd'
                            : winWidth > 500 ? 'w-tab'
                                : 'w-mob'
        ;
    document.body.className = `${w}`;
};

let _adsrCache = '';
const applyDynamicStyleRules = async function applyDynamicStyleRules() {
    let w = window;
    let h = await getEl('#app > header', 1) || { clientHeight: 0 };
    let m = await getEl('#app > main', 1) || { clientHeight: 0, clientWidth: 0 };
    let f = await getEl('#app > footer', 1) || { clientHeight: 0 };
    let dl = await getEl('aside.app-drawer.left .drawer-content') || { clientWidth: 0 };
    let dr = await getEl('aside.app-drawer.right .drawer-content') || { clientWidth: 0 };
    let value = `
:root {
--window-width: ${w.innerWidth}px;
--window-height: ${w.innerHeight}px;
--main-width: ${m.clientWidth}px;
--main-height: ${m.clientHeight}px;
--header-height: ${h.clientHeight}px;
--footer-height: ${f.clientHeight}px;
--drawer-left-width: ${dl.clientWidth}px;
--drawer-right-width: ${dr.clientWidth}px;
}
`;
    if (_adsrCache !== value) {
        _adsrCache = value;
        styles.innerHTML = value;
        set_body_class(w.innerWidth);
    }
};

(function periodicUpates(){
    applyDynamicStyleRules();
    setTimeout(periodicUpates, 1000);
})();

const getMatchByKey = function getMatchByKey(target, key) {
    let el = target;
    let i = 0;
    while (el && i++ < 10) {
        if (el[key]) { return el; }
        el = el.parentNode;
    }
    return undefined;
}
const handleNav = function handleNav(target) {
    if (!target.parentNode) {
        // WebUI Already removed from DOM
        return true;
    }
    let anchor = getMatchByKey(target, 'href');
    if (!anchor) { return false; }
    target = anchor.getAttribute('target');
    // All external links should get handled by Rust calling open_external_link
    if (target && target == '_blank') return true;
    let href = anchor.getAttribute('href');
    href = href.substr(0, location.origin.length).toLowerCase() === location.origin.toLowerCase() ? href.substr(location.origin.length) : href;
    if (href[0] === '#') {
        location.hash = href;
        return true;
    }
    // Disabling local navigation which will be handled by PWA webasembly processing
    if (href[0] === '/') {
        return true;
    }
    return false;
}
const styles = document.createElement('style');
const handlResize = function handlResize(ev) {
    applyDynamicStyleRules();
}

const setupWatchers = function setupWatchers() {
    styles.setAttribute('type', 'text/css');
    document.head.appendChild(styles);
    window.addEventListener('resize', handlResize);
    setTimeout(applyDynamicStyleRules,10);
}

getEl('#app', 30000).then(el => {
    if (!el) {
        console.error('WebUI Interop failed to load application');
        return;
    }
    app = el;
    loadAppClasses();
    el.addEventListener('click', ev => {
        if (!ev.target) { return false; }
        if (handleNav(ev.target)) {
            ev.preventDefault();
            ev.stopPropagation();
            return false;
        }
        return true;
    });
    setupWatchers();
    setTimeout(() => {
        update_app_classes("page transition in", "out");
        setTimeout(() => {
            remove_app_class("page transition in");
        }, 300);
    }, 200);
});

(function checkHighlighting(){
    if (window.hljs) {
        document.querySelectorAll('pre code:not([data-hl])').forEach((el) => {
            el.setAttribute('data-hl', true);
            window.hljs.highlightElement(el);
        });
    }
    setTimeout(checkHighlighting, 100);
})();

setTimeout(() => {
    setupTauriIntegrations();
}, 1000);
