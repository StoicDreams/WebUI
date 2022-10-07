'strict'
export function get_now_date() {
    console.log("get_now_date called!");
    var curr_date = new Date();
    return curr_date.toDateString();
}
