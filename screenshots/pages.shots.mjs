import { test, openPage, screenshotRoutes, pageScreenshotFile, pageRoles, signIn, expectSignedIn, PAGE_SCREENSHOTS } from '../tests/support/site.mjs';
import { existsSync, mkdirSync, readdirSync, statSync, unlinkSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

// Captures one screenshot of every page in nav.json, the site's main navigation, for marketing and design reviews.
// This file is the same in every site. Run it with PowerShell/screenshots.ps1, which Publish.ps1 runs.
// Each image is saved under webapp/root_files/img/screenshots/pages, named after its route ("/" is root.png),
// and an image whose page no longer exists is deleted.
//
// A page for signed-in visitors (a nav.json role above 0, or content all inside webui-restrict-to-role) is captured
// signed in as a test user with every role the page asks for (pageRoles in tests/support/site.mjs), so the picture
// shows what those visitors see rather than the sign-in prompt.
//
// A site can set up its pages in screenshots/pages.setup.mjs, which may export:
//   setup(page, route): runs before the page opens, for example to sign in or add sample data.
//   prepare(page, route): runs after the page opens and before the screenshot, for example to open a panel.

const setupFile = join(dirname(fileURLToPath(import.meta.url)), 'pages.setup.mjs');
const site = existsSync(setupFile) ? await import(pathToFileURL(setupFile).href) : {};
const routes = screenshotRoutes();

function removeOldScreenshots() {
    const keep = new Set(routes.map(pageScreenshotFile));
    const walk = dir => {
        if (!existsSync(dir)) return;
        for (const name of readdirSync(dir)) {
            const file = join(dir, name);
            if (statSync(file).isDirectory()) walk(file);
            else if (name.endsWith('.png') && !keep.has(file)) unlinkSync(file);
        }
    };
    walk(PAGE_SCREENSHOTS);
}
removeOldScreenshots();

// Each page starts from a fresh browser, so pages can be captured at the same time.
test.describe.configure({ mode: 'parallel' });

for (const route of routes) {
    test(`screenshot of ${route}`, async ({ page }) => {
        test.setTimeout(60000);
        // A fixed date and size, so the images only change when the UI does.
        await page.clock.setFixedTime(new Date('2026-11-02T15:00:00Z'));
        await page.setViewportSize({ width: 1280, height: 800 });
        const roles = pageRoles(route);
        if (roles !== 0) await signIn(page, roles);
        if (site.setup) await site.setup(page, route);
        await openPage(page, route);
        if (roles !== 0) await expectSignedIn(page, roles);
        if (site.prepare) await site.prepare(page, route);
        // Keep hover tooltips out of the picture: the corner is over the menu, so rest on the copyright line instead.
        // Then let fonts, images, and fades finish.
        await page.mouse.move(640, 798);
        await page.evaluate(() => document.fonts.ready);
        await page.waitForLoadState('networkidle');
        await page.waitForTimeout(400);
        const file = pageScreenshotFile(route);
        mkdirSync(dirname(file), { recursive: true });
        await page.screenshot({ path: file });
    });
}
