"use strict";

onmessage = async (msg) => {
    if (!msg.isTrusted) return;
    let data = JSON.parse(msg.data);
    let result = await processMessage(data.run, data.data);
    let response = { id: data.id, message: result };
    postMessage(JSON.stringify(response));
};

async function processMessage(run, data) {
    if (!run) return { ok: false, msg: 'Missing run' };
    let proc = procs[run];
    if (typeof proc === 'function') {
        try {
            return { ok: true, msg: await proc(data) };
        } catch (ex) {
            console.error('worker proc failed', ex);
            return { ok: false, msg: ex };
        }
    }
}

const procs = {
};