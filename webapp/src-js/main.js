window.addEventListener('DOMContentLoaded', () => {
    if (!window.Worker) {
        console.log("Web Workers are not supported.");
        return;
    }
    const myWorker = new Worker(new URL('/worker.js', import.meta.url), { type: 'module' });
    const messages = {};
    myWorker.onmessage = (msg) => {
        if (!msg.isTrusted) return;
        const { id, message } = msg.data;
        if (id) {
            messages[id] = message;
        }
    };
    const workerObject = new Object()
    const workerProxy = new Proxy(workerObject, {
        get(_,name) {
            let func = webui.toSnake(name).replace(/-/g,'_');
            return function(...args) {
                let msg = { id: webui.uuid(), run: func, data: args };
                return new Promise(async (resolve, reject) => {
                    myWorker.postMessage(msg);
                    let counter = 0;
                    while (counter++ < 60000) {
                        if (messages[msg.id]) {
                            let result = messages[msg.id];
                            delete messages[msg.id];
                            if (result.ok) {
                                resolve(result.msg);
                            } else {
                                reject(result.msg);
                            }
                            return;
                        }
                        await webui.wait(10);
                    }
                    reject('Message never returned');
                });
            }
        }
    })
    webui.worker = workerProxy;
    myWorker.onerror = (error) => {
        console.error('Main thread: Worker error:', error);
    };
});