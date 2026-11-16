import { defineConfig } from '@playwright/test';
import { existsSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

// Playwright does not provide browser builds for every Linux distribution, so prefer a
// system Chromium there. On other systems run: npx playwright install chromium
const systemChromium = process.platform === 'linux' && existsSync('/usr/bin/chromium') ? '/usr/bin/chromium' : undefined;
const executablePath = process.env.PW_CHROMIUM_PATH || systemChromium;

export default defineConfig({
    testDir: './tests',
    testMatch: '**/*.spec.mjs',
    fullyParallel: true,
    forbidOnly: !!process.env.CI,
    reporter: [['list'], ['html', { open: 'never' }]],
    use: {
        // Nothing listens here. Tests answer every request from webapp/ (see tests/support/site.mjs).
        baseURL: 'http://localhost:8114',
        serviceWorkers: 'block',
        screenshot: 'only-on-failure',
        trace: 'retain-on-failure',
        launchOptions: executablePath ? { executablePath } : {},
    },
    projects: [
        // The framework from the live CDN, which is what the deployed site uses.
        { name: 'live-cdn', use: { cdnDir: null } },
        // The framework from the MyFiCDN checkout in ../../Websites, to try unreleased framework changes.
        { name: 'local-cdn', use: { cdnDir: fileURLToPath(new URL('../../Websites/MyFiCDN/cdn', import.meta.url)) } },
    ],
});
