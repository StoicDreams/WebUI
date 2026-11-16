import { defineConfig } from '@playwright/test';
import base from './playwright.config.mjs';

// Captures screenshots instead of testing. Run it with PowerShell/screenshots.ps1, or npm run screenshots.
// Same browser and request handling as the tests, with the live CDN and no retries. Each script decides whether its
// pictures can be taken at the same time: pages.shots.mjs runs in parallel, and a script with one test runs in order.
// This file is the same in every site.
export default defineConfig({
    ...base,
    testDir: './screenshots',
    testMatch: '**/*.shots.mjs',
    fullyParallel: false,
    retries: 0,
    reporter: [['list']],
    outputDir: './test-results/screenshots',
    projects: base.projects.filter(project => project.name === 'live-cdn'),
});
