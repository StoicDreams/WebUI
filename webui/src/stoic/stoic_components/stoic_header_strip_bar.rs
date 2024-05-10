use crate::prelude::*;

pub fn stoic_header_strip_bar(_contexts: &Contexts) -> Html {
    html! {
        <>
            <span class="flex-break show-at-mobile" />
            <span class="flex-grow show-at-mobile" />
            <Paper>
                <Link class="btn theme-inherit pl-1 pr-1" icon={FaIcon::solid("badge-dollar")}
                    title="Sponser us to help support development"
                    href="https://github.com/sponsors/StoicDreams">
                </Link>
                <Link class="btn theme-inherit pl-1 pr-1" icon={FaIcon::brands("discord")}
                    title="Link to Stoic Dreams Discord server"
                    href="https://discord.com/channels/972856291909332993/1025781071608037466">
                </Link>
                <Link class="btn theme-inherit pl-1 pr-1" icon={FaIcon::brands("facebook")}
                    title="Link to Stoic Dreams on Facebook"
                    href="https://www.facebook.com/stoicdreams">
                </Link>
                <Link class="btn theme-inherit pl-1 pr-1" icon={FaIcon::brands("instagram")}
                    title="Link to Stoic Dreams on Instagram"
                    href="https://www.instagram.com/stoicdreamsllc">
                </Link>
            </Paper>
        </>
    }
}
