# Tests

Playwright tests for this site. They need no build and no server: every request to the site is answered from `webapp/`, laid out the way Trunk copies it. Analytics and other outside requests are blocked, and `api.myfi.ws` returns an empty response.

```bash
npm install
npm test
```

- `npm test` loads the framework from the live CDN, which is what the deployed site uses.
- `npm run test:local-cdn` loads it from the `MyFiCDN` checkout in `../../Websites/MyFiCDN`, to try unreleased framework changes.
- `npm run test:ui` opens the Playwright UI.

## Files

- `site.spec.mjs` and `support/site.mjs` are the same in every site. Change them in every site at once.
- `service-worker.spec.mjs` and `scripts.spec.mjs` are the same in every site, the CDN, and Task Proxy. They test the service worker and the `PowerShell` scripts, and the script tests need PowerShell 7 (`pwsh`).
- `site.config.mjs` holds this site's settings. A shared check this site is known to fail goes in `knownBugs`, so the suite stays green and the test flags itself once fixed.
- Add checks for this site's own pages and components in new `*.spec.mjs` files, using `test`, `expect`, and `openPage` from `support/site.mjs`.

Rust code in `webapp/src` is tested with `cargo test`. Components that call `webui.worker` need `webapp/pkg`, which `PowerShell/RunLocal.ps1` or `wasm-pack build --target web --out-dir pkg` (run in `webapp`) creates.
