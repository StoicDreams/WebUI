<webui-data data-page-title="Starter Website Files" data-page-subtitle=""></webui-data>

### Starter Website Files

<webui-page-segment elevation="10">
    The files below are the basic files needed to create a starter website using Stoic Dreams' Web UI framework.
    You can use these files for the starting boilerplate to create your own website.
</webui-page-segment>

```html:index.html
<!DOCTYPE html>
<html lang="en">
<head>
    <title>Starter Demo</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="robots" content="index, follow">
    <base href="/" />
    <script type="text/javascript">
        navigator.serviceWorker.register('service-worker.js');
    </script>
    <link rel="dns-prefetch" href="https://api.myfi.ws" />
    <link href="https://cdn.myfi.ws/css/webui.min.css" rel="preload" as="style" />
    <link href="https://cdn.myfi.ws/webui/loader.min.js" rel="preload" as="script" />
    <link href="Logo.svg" rel="icon" type="image/svg+xml" sizes="any" />
    <link rel="manifest" href="app.webmanifest">
    <link href="https://cdn.myfi.ws/css/webui.min.css" rel="stylesheet" />
    <script src="https://cdn.myfi.ws/webui/loader.min.js" async></script>
    <meta name="theme-color" content="#FF2E46" />
    <meta name="author" content="Stoic Dreams">
    <meta name="description" content="Edit this description to match your website.">
    <meta property="og:image" content="Logo.svg">
    <meta property="og:title" content="Stoic Dreams content host and Web UI testing playground">
    <meta property="og:url" content="https://localhost/">
    <meta property="og:description" content="Edit this description to match your website.">
</head>
<body>
    <webui-app-config src="appConfig.json"></webui-app-config>
    <webui-data>
        <template slot="html" name="app-not-found">
            <webui-flex grow></webui-flex>
            <p>This is custom 404 content set by a webui-data component in the index.html file.</p>
            <p class="text-center"><webui-link theme="info" href="/">Return Home</webui-link></p>
            <webui-flex grow></webui-flex>
        </template>
        <template slot="html" name="app-under-construction">
            <p>This is custom Under Construction content set by a webui-data component in the index.html file.</p>
            <p class="text-center"><webui-link theme="success" href="/">Return Home</webui-link></p>
        </template>
    </webui-data>
    <webui-app data-removeclass=".nav|open;.shared|open">
        <webui-drawer slot="left" class="nav elevation-20" docked="true" data-state="slot|docked" data-moveable data-dockable>
            <webui-flex justify="center" slot="header">
                <webui-stoic-dreams-logo title="Starter Logo" text="Site" text2="Demo"></webui-stoic-dreams-logo>
            </webui-flex>
            <webui-nav routes="/nav.json" data-subscribe="app-nav-routes:setNavRoutes"></webui-nav>
        </webui-drawer>
        <webui-drawer slot="right" class="shared elevation-20" data-stopclick data-moveable data-state="slot">
        </webui-drawer>
        <header slot="header">
            <button aria-label="open navigation menu" data-toggleclass=".nav|open" class="elevation-10 pa-1 mx-1">
                <webui-icon icon="hamburger"></webui-icon>
            </button>
            <h1 data-subscribe="page-title:innerHTML">Website Starter</h1>
            <h2 data-subscribe="page-subtitle:innerHTML"></h2>
            <webui-flex grow></webui-flex>
            <webui-feedback theme="primary" flags="shape:circle backing border" title="Provide us your feedback" data-post="https://api.myfi.ws/feedback/new" data-json-name="message"></webui-feedback>
            <webui-alerts title="View your notifications" data-title="My Alerts" data-toggleclass=".shared|open"></webui-alerts>
            <webui-myfi-info>
                <template slot="panel-content">
                    <webui-alert show variant="info">Example Account Panel</webui-alert>
                </template>
            </webui-myfi-info>
        </header>
        <noscript>Javascript is required to view this site</noscript>
        <webui-page-footer copyright="2025" company="Your Company">
            <webui-line></webui-line>
            <webui-next-page theme="tertiary" data-subscribe="page-next-page"></webui-next-page>
            <webui-flex class="my-3" justify="center" wrap>
                <webui-link href="/about" icon="exclamation|shape:circle|backing|bordered">About {APP_NAME}</webui-link>
                <webui-link href="/contact" icon="messages|fill|shade:tri">Contact {COMPANY_SINGULAR}</webui-link>
                <webui-link href="/privacy" icon="exclamation|shape:shield|backing|bordered">Privacy</webui-link>
                <webui-link href="/terms" icon="handshake|fill|shade|tri">Terms & Conditions</webui-link>
            </webui-flex>
            <webui-stoic-social-links class="mb-2"></webui-stoic-social-links>
        </webui-page-footer>
    </webui-app>
</body>
</html>
```

```javascript:service-worker.js
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
    try {
        return fetch(request);
    } catch (ex) {
        console.error(ex);
        return Response.error();
    }
}
```
