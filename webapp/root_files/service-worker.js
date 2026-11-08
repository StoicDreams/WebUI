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
const currentVersion = '0.11.8';
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
/**
 * Network first, so a CDN deploy reaches the app on its next load. The cache is only a fallback for when the
 * network fails, such as when offline, or answers with an error.
 */
async function onFetch(event) {
    const cache = await caches.open(cacheName);
    let networkResponse;
    try {
        networkResponse = await fetch(applyCacheBusting(event.request));
    } catch (error) {
        const cachedResponse = await cache.match(event.request);
        if (cachedResponse) return cachedResponse;
        throw error;
    }
    if (networkResponse.ok) {
        event.waitUntil(cache.put(event.request, networkResponse.clone()).catch(() => { }));
        return networkResponse;
    }
    return (await cache.match(event.request)) || networkResponse;
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
