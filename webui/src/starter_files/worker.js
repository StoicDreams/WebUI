import wasmInit, * as procs from '/webapp.js';
const AsyncFunction = (async () => { }).constructor;

// Listen for messages from the main thread
self.onmessage = async (msg) => {
    if (!msg.isTrusted) return;
    const { id, run, data } = msg.data;
    let result = await processMessage(run, data);
    let response = { id: id, message: result };
    postMessage(response);
};

async function processMessage(run, data) {
    if (!run) return { ok: false, msg: 'Missing run' };
    let proc = procs[run];
    if (typeof proc === 'function') {
        try {
            if (proc.constructor == AsyncFunction) {
                return { ok: true, msg: await proc(...data) };
            } else {
                return { ok: true, msg: proc(...data) };
            }
        } catch (ex) {
            console.error('worker proc failed', ex);
            return { ok: false, msg: ex };
        }
    }
}

// Optional: Handle errors originating from the worker itself
self.onerror = (error) => {
    console.error("Unhandled error in worker:", error);
};

wasmInit();
