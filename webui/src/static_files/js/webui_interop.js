"use strict"

export function log(message) {
    console.log(message, TimeStamp());
}
function TimeStamp() {
    const now = Date.now();
    const milliseconds = Math.floor(now % 1000);
    const seconds = Math.floor((now / 1000) % 60);
    const minutes = Math.floor((now / 1000 / 60) % 60);
    const hours = Math.floor(now / 1000 / 60 / 60 % 24);

    return [
        pad(hours),
        pad(minutes),
        pad(seconds),
        pad(milliseconds)
    ].join(':');
    function pad(number) { return number < 10 ? `0${number}` : number; }
}

const GLOBALDATA = {};
export function set_global_data(key, data) {
    GLOBALDATA[key] = data;
}

export function get_global_data(key) {
    return GLOBALDATA[key] ?? '';
}

export function get_uuid() {
    try {
        return crypto.randomUUID();
    } catch (ex) {
        return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
            let r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
            return v.toString(16);
        });
    }
}
