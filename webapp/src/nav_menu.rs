use crate::prelude::*;

use crate::pages::*;

pub fn nav_content(contexts: &Contexts) -> Html {
    html! {
        <>
            <section class="d-flex justify-center" slot="header">
                <webui-stoic-dreams-logo title="WebUI Logo" text="Web" text2="UI"></webui-stoic-dreams-logo>
            </section>
            <NavDisplay routes={get_nav_routing(contexts)} class="d-flex flex-column pa-1" />
        </>
    }
}

pub(crate) fn get_nav_routing(contexts: &Contexts) -> Vec<NavRoute> {
    let nav_routes = vec![
        NavLinkInfo::link(
            "Home",
            "/",
            &FaIcon::duotone("house"),
            roles::PUBLIC,
            page_home,
        ),
        NavGroupInfo::link(
            "Stoic Dreams Admin",
            &FaIcon::duotone("jedi"),
            roles::SITE_ADMIN,
            vec![NavLinkInfo::link(
                "Site Admin",
                "/site_admin",
                &FaIcon::duotone("jedi"),
                roles::SITE_ADMIN,
                page_site_admin_home,
            )],
        ),
        NavGroupInfo::link(
            "Classes",
            &FaIcon::duotone("file-code"),
            roles::PUBLIC,
            vec![
                NavLinkInfo::link(
                    "Helpers",
                    "/classes/helpers",
                    &FaIcon::brands("css3"),
                    roles::PUBLIC,
                    page_classes_helpers,
                ),
                NavLinkInfo::link(
                    "Themes",
                    "/classes/themes",
                    &FaIcon::duotone("palette"),
                    roles::PUBLIC,
                    page_classes_themes,
                ),
                NavLinkInfo::link(
                    "Variables",
                    "/classes/variables",
                    &FaIcon::brands("rust"),
                    roles::PUBLIC,
                    page_classes_variables,
                ),
            ],
        ),
        NavGroupInfo::link(
            "Components",
            &FaIcon::duotone("toolbox"),
            roles::PUBLIC,
            vec![
                NavLinkInfo::link(
                    "Containers",
                    "/components/containers",
                    &FaIcon::duotone("box-open-full"),
                    roles::PUBLIC,
                    page_components_containers,
                ),
                NavLinkInfo::link(
                    "Display",
                    "/components/display",
                    &FaIcon::duotone("photo-film"),
                    roles::PUBLIC,
                    page_components_display,
                ),
                NavLinkInfo::link(
                    "Touch",
                    "/components/touch",
                    &FaIcon::duotone("hand-back-point-up"),
                    roles::PUBLIC,
                    page_components_touch,
                ),
            ],
        ),
        NavGroupInfo::link(
            "Blogs",
            &FaIcon::duotone("blog"),
            roles::PUBLIC,
            vec![NavLinkInfo::link(
                "What is a web framework?",
                "/blogs/what-is-a-website-framework",
                &FaIcon::duotone("block-question"),
                roles::PUBLIC,
                page_blogs_what_is_ui_framework,
            )],
        ),
        NavGroupInfo::link(
            "Starter Pages",
            &FaIcon::duotone("file-code"),
            roles::PUBLIC,
            vec![NavLinkInfo::link(
                "Under Construction",
                "/under-construction",
                &FaIcon::duotone("traffic-cone"),
                roles::PUBLIC,
                starter_page_under_construction,
            )],
        ),
        NavLinkInfo::link(
            "About",
            "/about",
            &FaIcon::duotone("circle-info"),
            roles::PUBLIC,
            page_about,
        ),
        NavLinkInfo::link(
            "Terms",
            "/terms",
            &FaIcon::duotone("handshake"),
            roles::PUBLIC,
            starter_page_terms,
        ),
        NavLinkInfo::link(
            "Privacy",
            "/privacy",
            &FaIcon::duotone("shield-exclamation"),
            roles::PUBLIC,
            starter_page_privacy,
        ),
    ];
    nav_routes
}
