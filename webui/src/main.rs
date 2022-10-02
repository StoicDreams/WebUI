
mod build;

static CSS: &str = include_str!("css/webui.css");
static INDEX: &str = include_str!("index.html");

fn main() {
    build::add_file("css/webui.css", CSS);
	build::add_file("index.html", INDEX);
}
