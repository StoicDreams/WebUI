self.addEventListener('install', event => event.waitUntil(onInstall(event)));
self.addEventListener('activate', event => event.waitUntil(onActivate(event)));
self.addEventListener('fetch', event => event.respondWith(onFetch(event)));
function get_uuid() {
	try {
		return crypto.randomUUID();
	} catch (ex) {
		return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
			let r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
			return v.toString(16);
		});
	}
}
const cacheNamePrefix = 'offline-cache';
const cachePostfix = location.host.startsWith('localhost') ? `_${get_uuid()}` : '_ts_2507081608';
const cacheName = `${cacheNamePrefix}${cachePostfix}`;
const cdnCacheTimestamps = new Map();
const CACHE_REFRESH_INTERVAL = 60 * 60 * 1000;

async function onInstall(event) {
	console.info(`Service worker: Install ${cacheName}`);
	self.skipWaiting();
}

async function onActivate(event) {
	console.info(`Service worker: Activate ${cacheName}`);
	const cacheKeys = await caches.keys();
	await Promise.all(cacheKeys
		.filter(key => key.startsWith(cacheNamePrefix) && key !== cacheName)
		.map(key => caches.delete(key)));
}

async function onFetch(event) {
	const request = event.request;
	const url = new URL(request.url);
	if (url.hostname === 'api.myfi.ws') {
		return fetch(request, { cache: 'no-store' });
	}
	if (url.hostname === 'cdn.myfi.ws' && request.method === 'GET') {
		const cache = await caches.open(cacheName);
		const cachedResponse = await cache.match(request);
		const now = Date.now();
		const lastFetched = cdnCacheTimestamps.get(request.url) || 0;
		if (cachedResponse && (now - lastFetched) < CACHE_REFRESH_INTERVAL) {
			return cachedResponse;
		}
		try {
			const freshResponse = await fetch(request);
			if (freshResponse.ok) {
				cache.put(request, freshResponse.clone());
				cdnCacheTimestamps.set(request.url, now);
			}
			return freshResponse;
		} catch (error) {
			return cachedResponse || Response.error();
		}
	}
	return fetch(request);
}
