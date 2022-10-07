'strict'
export function get_now_date() {
    console.log("get_now_date called!");
    var curr_date = new Date();
    return curr_date.toDateString();
}

export function get_path() {
    return get_full_path().split('?')[0];
}

export function get_full_path() {
    return location.href.substring(location.origin.length);
}

export function log(message) {
    console.log(message);
}