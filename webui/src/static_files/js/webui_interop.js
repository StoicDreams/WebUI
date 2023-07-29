"use strict"

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
    let cp = location.pathname.toLowerCase().split('?')[0].split('#')[0];
    if (path == cp) { return; }
    let app = document.getElementById('app');
    app.className = 'page transition out';
    history.pushState(null, null, path);
    let halftran = pageTransitionDuration / 2;
    setTimeout(() => {
        app.className = 'page transition in';
        setTimeout(() => {
            app.className = '';
        }, halftran);
    }, halftran);
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
            return memStorageCache[key];
        }
        acceptLocalStorage() {
            acceptedStorage = ACCEPT_LOCAL_STORAGE;
            sessionStorage.clear();
            Object.keys(memStorageCache).forEach(key => {
                localStorage.setItem(key, memStorageCache[key]);
            });
        }
        acceptSessionStorage() {
            acceptedStorage = ACCEPT_SESSION_STORAGE;
            localStorage.clear();
            Object.keys(memStorageCache).forEach(key => {
                sessionStorage.setItem(key, memStorageCache[key]);
            });
        }
        rejectCachedStorage() {
            acceptedStorage = REJECT_STORAGE_CACHING;
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

export async function webui_fetch(url, jsonIn) {
    let options = JSON.parse(jsonIn);
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
