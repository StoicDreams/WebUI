use yew::prelude::*;

pub(crate) struct App;

impl Component for App {
	type Message = ();
	type Properties = ();

	fn create(ctx: &Context<Self>) -> Self {
		App
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		html! {
			<div id="app">
				<header>
					{"Web UI"}
				</header>
				<main>
					{"Page Body"}
				</main>
				<footer>
				</footer>
				<aside class="top"></aside>
				<aside class="right"></aside>
				<aside class="bottom"></aside>
				<aside class="left"></aside>
			</div>
		}
	}
}