use webui::*;

pub(crate) fn header_strip_bar() -> Html {
    html! {
        <Paper>
            <Link class="btn theme-inherit pl-1 pr-1" icon="fa-solid fa-badge-dollar"
                title="Sponser us to help support development"
                href="https://github.com/sponsors/StoicDreams">
            </Link>
            <Link class="btn theme-inherit pl-1 pr-1" icon="fa-brands fa-facebook"
                title="Link to Stoic Dreams on Facebook"
                href="https://www.facebook.com/stoicdreams">
            </Link>
            <Link class="btn theme-inherit pl-1 pr-1" icon="fa-brands fa-instagram"
                title="Link to Stoic Dreams on Instagram"
                href="https://www.instagram.com/stoicdreamsllc">
            </Link>
        </Paper>
    }
}
