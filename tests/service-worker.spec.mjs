import { test, expect } from '@playwright/test';
import { existsSync, readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import vm from 'node:vm';

// The service worker is the same in every site, the CDN, Task Proxy, and the Web UI starter files, so change this spec in all of them at once.
// It runs the worker in a sandbox with stand-ins for the browser's caches and fetch, since Playwright blocks real service workers.
const candidates = ['../webapp/root_files/service-worker.js', '../cdn/service-worker.js', '../root_files/service-worker.js', '../../root_files/service-worker.js'];
const sourcePath = candidates.map(path => fileURLToPath(new URL(path, import.meta.url))).find(path => existsSync(path));
const versionOf = text => text.match(/\bcurrentVersion\s*=\s*['"]([^'"]+)['"]/)?.[1];
const source = readFileSync(sourcePath, 'utf8');
const version = versionOf(source);
const cacheName = `offline-cache-${version}`;
const site = 'https://example.com';

function loadWorker(file, network) {
    const listeners = {};
    const stores = new Map();
    const fetched = [];
    const caches = {
        async keys() { return [...stores.keys()]; },
        async delete(name) { return stores.delete(name); },
        async open(name) {
            if (!stores.has(name)) stores.set(name, new Map());
            const store = stores.get(name);
            return {
                async match(request) { return store.get(request.url)?.clone(); },
                async put(request, response) { store.set(request.url, response); },
            };
        },
    };
    const self = {
        addEventListener(type, listener) { listeners[type] = listener; },
        skipWaiting() { self.skippedWaiting = true; },
        clients: { async claim() { self.claimedClients = true; } },
    };
    const fetch = async request => {
        fetched.push(request.url);
        return network(request);
    };
    vm.runInContext(readFileSync(file, 'utf8'), vm.createContext({ self, caches, fetch, Request, Response, URL, Date, Promise }));
    return {
        self, stores, fetched,
        cached(url) { return stores.get(cacheName)?.get(url); },
        async seed(url, text) { await (await caches.open(cacheName)).put({ url }, new Response(text)); },
        // Sends a fetch event, and returns what the worker answered with, or undefined when it let the browser handle it.
        async request(request) {
            let answer;
            const waits = [];
            listeners.fetch({ request, respondWith(promise) { answer = promise; }, waitUntil(promise) { waits.push(promise); } });
            const result = answer && await answer;
            await Promise.all(waits);
            return result;
        },
        async lifecycle(type) {
            const waits = [];
            listeners[type]({ waitUntil(promise) { waits.push(promise); } });
            await Promise.all(waits);
        },
    };
}

const online = text => async () => new Response(text);
const offline = async () => { throw new TypeError('Failed to fetch'); };
const serverError = async () => new Response('broken', { status: 500 });

test('the minified worker has the same version as the source', () => {
    const minPath = sourcePath.replace(/\.js$/, '.min.js');
    expect(existsSync(minPath), 'service-worker.min.js is missing').toBe(true);
    expect(version, 'service-worker.js should set currentVersion').toBeTruthy();
    expect(versionOf(readFileSync(minPath, 'utf8')), 'Regenerate service-worker.min.js').toBe(version);
});

for (const [label, file] of [['service-worker.js', sourcePath], ['service-worker.min.js', sourcePath.replace(/\.js$/, '.min.js')]]) {
    test.describe(label, () => {
        test('a file comes from the network even when it is cached, skipping the browser cache', async () => {
            const worker = loadWorker(file, online('new'));
            await worker.seed(`${site}/app.js`, 'old');
            const response = await worker.request(new Request(`${site}/app.js`));
            expect(await response.text()).toBe('new');
            expect(worker.fetched).toHaveLength(1);
            expect(new URL(worker.fetched[0]).searchParams.get('_')).toMatch(/^\d+$/);
        });

        test('a file from the network is cached under its own address', async () => {
            const worker = loadWorker(file, online('fresh'));
            await worker.request(new Request(`${site}/app.js?v=1`));
            expect(await worker.cached(`${site}/app.js?v=1`)?.text()).toBe('fresh');
            expect(worker.stores.get(cacheName).size).toBe(1);
        });

        test('offline, the cached copy is used', async () => {
            const worker = loadWorker(file, offline);
            await worker.seed(`${site}/app.js`, 'cached');
            expect(await (await worker.request(new Request(`${site}/app.js`))).text()).toBe('cached');
        });

        test('offline with nothing cached, the request fails', async () => {
            const worker = loadWorker(file, offline);
            await expect(worker.request(new Request(`${site}/app.js`))).rejects.toThrow('Failed to fetch');
        });

        test('an error from the server falls back to the cached copy, and is not cached', async () => {
            const worker = loadWorker(file, serverError);
            await worker.seed(`${site}/app.js`, 'cached');
            expect(await (await worker.request(new Request(`${site}/app.js`))).text()).toBe('cached');
            expect(await worker.cached(`${site}/app.js`).text()).toBe('cached');
        });

        test('an error from the server with nothing cached is passed on', async () => {
            const worker = loadWorker(file, serverError);
            const response = await worker.request(new Request(`${site}/app.js`));
            expect(response.status).toBe(500);
            expect(worker.cached(`${site}/app.js`)).toBeUndefined();
        });

        test('pages, other methods, local and app addresses, and non-web requests are left to the browser', async () => {
            const worker = loadWorker(file, online('x'));
            const ignored = [
                { url: `${site}/about`, method: 'GET', mode: 'navigate' },
                new Request(`${site}/api`, { method: 'POST', body: '{}' }),
                new Request('http://localhost:8080/app.js'),
                new Request('http://127.0.0.1:1426/app.js'),
                new Request('http://tauri.localhost/app.js'),
                new Request('http://ipc.localhost/command'),
                { url: 'ipc://localhost/command', method: 'GET', mode: 'cors' },
                { url: 'data:text/plain,x', method: 'GET', mode: 'cors' },
            ];
            for (const request of ignored) {
                expect(await worker.request(request), request.url).toBeUndefined();
            }
            expect(worker.fetched).toEqual([]);
        });

        test('installing takes over without waiting for old tabs to close', async () => {
            const worker = loadWorker(file, online('x'));
            await worker.lifecycle('install');
            expect(worker.self.skippedWaiting).toBe(true);
        });

        test('activating deletes caches from other versions, keeps other caches, and takes control of open tabs', async () => {
            const worker = loadWorker(file, online('x'));
            for (const name of ['offline-cache-0.0.1', cacheName, 'someone-elses-cache']) worker.stores.set(name, new Map());
            await worker.lifecycle('activate');
            expect([...worker.stores.keys()].sort()).toEqual([cacheName, 'someone-elses-cache'].sort());
            expect(worker.self.claimedClients).toBe(true);
        });
    });
}
