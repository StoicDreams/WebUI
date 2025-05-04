// src-js/main.js

window.addEventListener('DOMContentLoaded', () => {
    const resultArea = document.getElementById('resultArea');
    const processButton = document.getElementById('processButton');
    const addButton = document.getElementById('addButton');
    const statusArea = document.getElementById('statusArea');

    if (!window.Worker) {
        statusArea.textContent = "Web Workers are not supported.";
        return;
    }

    // Instantiate the worker using the source path.
    // Trunk will process 'src-js/worker.js' because of the <link> in index.html
    // and rewrite this path during build time to point to the bundled worker.
    // Use { type: 'module' } as Trunk outputs workers as JS modules.
    const myWorker = new Worker(new URL('./worker.js', import.meta.url), { type: 'module' });
    // Using `new URL(...)` is the modern, robust way to specify worker paths relative
    // to the current module, which Trunk handles correctly.

    statusArea.textContent = 'Worker loading...';
    console.log('Main thread: Worker created.');

    // No changes needed for the rest of the logic:
    // onmessage handling, onerror handling, postMessage calls

    myWorker.onmessage = (event) => {
        const { type, payload } = event.data;
        switch (type) {
            // NOTE: We removed the WORKER_READY message type, as Trunk handles init.
            // The worker is effectively ready once the script loads.
            // You might want a different signal if the worker has async setup *beyond* WASM loading.

            case 'RESULT_ADD':
                resultArea.textContent = `Addition Result: ${payload}`;
                statusArea.textContent = 'Worker task finished.';
                break;

            case 'RESULT_PROCESS_DATA':
                resultArea.textContent = `Processing Result:\n${payload}`;
                statusArea.textContent = 'Worker task finished.';
                break;

            case 'WORKER_ERROR':
                statusArea.textContent = `Worker Error: ${payload}`;
                console.error('Worker Error:', payload);
                break;

            default:
                console.warn('Main thread received unknown message type:', type);
        }
    };

    myWorker.onerror = (error) => {
        statusArea.textContent = `Worker error occurred: ${error.message}`;
        console.error('Main thread: Worker error:', error);
    };

    // --- Communication TO Worker ---
    processButton.onclick = () => {
        const dataToSend = document.getElementById('inputText').value || "Default Data";
        const intensity = parseInt(document.getElementById('intensity').value, 10) || 5;
        statusArea.textContent = 'Sending data to worker...';
        resultArea.textContent = '';
        myWorker.postMessage({
            type: 'PROCESS_DATA',
            payload: { data: dataToSend, intensity: intensity }
        });
    };

    addButton.onclick = () => {
        const a = parseInt(document.getElementById('numA').value, 10) || 0;
        const b = parseInt(document.getElementById('numB').value, 10) || 0;
        statusArea.textContent = 'Sending numbers to worker...';
        resultArea.textContent = '';
        myWorker.postMessage({
            type: 'ADD',
            payload: { a: a, b: b }
        });
    };

    // The worker should be ready almost immediately after creation now,
    // as Trunk ensures WASM is loaded. You could enable buttons right away,
    // or wait for a first message if needed for other reasons.
    statusArea.textContent = 'Worker ready (WASM handled by Trunk).';

}); // End DOMContentLoaded