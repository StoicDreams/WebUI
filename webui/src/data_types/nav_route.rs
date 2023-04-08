use crate::prelude::*;

/// Page navigation information
///
/// This data struct hold information for a single page within your app.
/// Roles are expected to be defined using powers of 2 (1, 2, 4, 8, 16, 32, 64, etc.).
/// 0 is reserved for Guest or Public, meaning everyone has permissions to this page.
///
/// example
/// ```rust
/// use webui::*;
///
/// fn page_home() -> Html {
///     html! {
///         {"Home Page"}
///     }
/// }
///
/// let link_info = NavLinkInfo::new("Home", "/", "fa-solid fa-bars", roles::PUBLIC, page_home);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct NavLinkInfo {
    pub name: String,
    pub path: String,
    pub icon: String,
    pub role: u32,
    pub page: fn() -> Html,
}

impl NavLinkInfo {
    /// Create a new instance of NavLinkInfo
    ///
    /// example
    /// ```rust
    /// use webui::*;
    ///
    /// fn page_home() -> Html {
    ///     html! {
    ///         {"Home Page"}
    ///     }
    /// }
    ///
    /// let link_info = NavLinkInfo::new("Home", "/", "fa-solid fa-bars", roles::PUBLIC, page_home);
    /// ```
    pub fn new(name: &str, path: &str, icon: &str, role: u32, page: fn() -> Html) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            icon: icon.to_string(),
            role,
            page,
        }
    }

    pub fn link(name: &str, path: &str, icon: &str, role: u32, page: fn() -> Html) -> NavRoute {
        NavRoute::NavLink(Self {
            name: name.to_string(),
            path: path.to_string(),
            icon: icon.to_string(),
            role,
            page,
        })
    }
}

/// Grouping of page navigation
///
/// Used by navigation displays to organize subfolders in navigation tree.
/// Roles are expected to be defined using powers of 2 (1, 2, 4, 8, 16, 32, 64, etc.).
///
/// Roles in NavGroupInfo do not affect page permissions, they are only used for navigation displays.
/// 0 is reserved for Guest or Public, meaning everyone has permissions to see the group in navigation.
///
/// A use case where you would want to set the NavGroupInfo::role value higher than child pages is when
/// you want a page to be accessible but not navigable from the navigation tree.
/// For example, a confirmation page after a user action is completed.
#[derive(Debug, Clone, PartialEq)]
pub struct NavGroupInfo {
    pub name: String,
    pub icon: String,
    pub role: u32,
    pub children: Vec<NavRoute>,
}

impl NavGroupInfo {
    /// Create a new instance of NavLinkInfo
    ///
    /// example
    /// ```rust
    /// use webui::*;
    ///
    /// fn page_home() -> Html {
    ///     html! {
    ///         {"Home Page"}
    ///     }
    /// }
    ///
    /// let link_info = NavGroupInfo::new("Home", "fa-solid fa-bars", roles::PUBLIC, vec![
    ///     NavRoute::NavLink(NavLinkInfo::new("Home", "/", "fa-solid fa-bars", roles::PUBLIC, page_home))
    /// ]);
    /// ```
    pub fn new(name: &str, icon: &str, role: u32, children: Vec<NavRoute>) -> Self {
        Self {
            name: name.to_string(),
            icon: icon.to_string(),
            role,
            children,
        }
    }

    pub fn link(name: &str, icon: &str, role: u32, children: Vec<NavRoute>) -> NavRoute {
        NavRoute::NavGroup(Self {
            name: name.to_string(),
            icon: icon.to_string(),
            role,
            children,
        })
    }
}

/// Enum types for navigation data
///
/// NavLink represents page information
/// NavGroup represents groupings of page information
///
/// example
/// ```rust
/// use webui::*;
///
/// fn page_home() -> Html {
///     html! {
///         {"Home Page"}
///     }
/// }
///
/// fn page_about() -> Html {
///     html! {
///         {"About Page"}
///     }
/// }
///
/// pub(crate) fn get_nav_routing() -> Vec<NavRoute> {
///     let nav_routes: &mut Vec<NavRoute> = &mut Vec::new();
///     nav_routes.push(NavRoute::NavLink(NavLinkInfo::new("Home", "/", "fa-solid fa-bars", roles::PUBLIC, page_home)));
///     nav_routes.push(NavRoute::NavLink(NavLinkInfo::new("About", "/about", "fa-solid fa-circle-info", roles::PUBLIC, page_about)));
///     nav_routes.to_owned()
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum NavRoute {
    NavLink(NavLinkInfo),
    NavGroup(NavGroupInfo),
}
