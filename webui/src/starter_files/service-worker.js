self.addEventListener('install', event => event.waitUntil(onInstall(event)));
self.addEventListener('activate', event => event.waitUntil(onActivate(event)));
self.addEventListener('fetch', event => {
    const url = new URL(event.request.url);
    if (
        url.protocol === 'ipc:' ||
        url.protocol === 'tauri:' ||
        url.hostname === 'ipc.localhost' ||
        url.hostname === 'tauri.localhost' ||
        url.hostname.startsWith('127') ||
        url.hostname === 'localhost'
    ) {
        return;
    }
    if (!event.request.url.startsWith('http')) {
        return;
    }
    if (!allowCache(event.request)) {
        return;
    }
    event.respondWith(onFetch(event))
});
const currentVersion = '0.1.0';
const cacheNamePrefix = 'offline-cache-';
const cacheName = `${cacheNamePrefix}${currentVersion}`;
async function onInstall(event) {
    self.skipWaiting();
}
async function onActivate(event) {
    const cacheKeys = await caches.keys();
    await Promise.all(cacheKeys
        .filter(key => key.startsWith(cacheNamePrefix) && key !== cacheName)
        .map(key => caches.delete(key)));
    await self.clients.claim();
}
async function onFetch(event) {
    let request = applyCacheBusting(event.request);
    const cache = await caches.open(cacheName);
    const cachedResponse = await cache.match(event.request);
    const networkFetchPromise = fetch(request).then(networkResponse => {
        if (networkResponse && networkResponse.ok) {
            cache.put(event.request, networkResponse.clone());
        }
        return networkResponse;
    }).catch(error => {
        console.error('Background fetch failed:', error);
    });
    if (cachedResponse) {
        event.waitUntil(networkFetchPromise);
        return cachedResponse;
    }
    return networkFetchPromise;
}
function applyCacheBusting(request) {
    try {
        const url = new URL(request.url);
        url.searchParams.set('_', Date.now());
        return new Request(url.toString(), request);
    } catch {
        return request;
    }
}
function allowCache(request) {
    if (request.method !== 'GET') { return false; }
    if (request.mode === 'navigate') { return false; }
    return true;
}
