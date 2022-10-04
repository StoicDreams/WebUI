self.addEventListener('install', event => event.waitUntil(onInstall(event)));
self.addEventListener('activate', event => event.waitUntil(onActivate(event)));
self.addEventListener('fetch', event => event.respondWith(onFetch(event)));

const currentVersion = 'V001';
const cacheNamePrefix = 'offline-cache-';
const cacheName = `${cacheNamePrefix}${currentVersion}`;
const offlineAssetsInclude = [/\.wasm/, /\.html/, /\.js$/, /\.json$/, /\.css$/, /\.woff$/, /\.png$/, /\.jpe?g$/, /\.gif$/, /\.ico$/];
const offlineAssetsExclude = [/^service-worker\.js$/];

async function onInstall(event) {
	console.info('Service worker: Install');
}

async function onActivate(event) {
	console.info('Service worker: Activate');

	// Delete unused caches
	const cacheKeys = await caches.keys();
	await Promise.all(cacheKeys
		.filter(key => key.startsWith(cacheNamePrefix) && key !== cacheName)
		.map(key => caches.delete(key)));
}

async function onFetch(event) {
	let cachedResponse = null;
	if (allowCache(event.request)) {
		const cache = await caches.open(cacheName);
		cachedResponse = await cache.match(event.request);
	}

	return cachedResponse || fetch(event.request);
}

function allowCache(request) {
	// Only allow caching for GET requests
	if (request.method !== 'GET') { return false; }
	// Exclude caching for navigation requests to ensure the latest site updates are loaded asap
	if (request.mode === 'navigate') { return false; }
	// All other GET requests allow navigation
	return true;
}
