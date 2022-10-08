use crate::*;

/// Struct used for defining details for displaying buttons that toggle drawer content.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawerToggleInfo {
    pub(crate) display: fn() -> Html,
    pub(crate) title: String,
    pub(crate) class: String,
    pub(crate) drawer: Direction,
    pub(crate) drawer_content: fn() -> Html,
    pub(crate) hide_title: bool,
    pub(crate) hide_close_x: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DrawerToggleInfoBuilder {
    display: fn() -> Html,
    title: String,
    class: String,
    drawer: Direction,
    drawer_content: fn() -> Html,
    hide_title: bool,
    hide_close_x: bool,
}

impl DrawerToggleInfo {
    pub fn new(
        title: String,
        button_display: fn() -> Html,
        drawer_content: fn() -> Html,
    ) -> DrawerToggleInfoBuilder {
        DrawerToggleInfoBuilder {
            title,
            display: button_display,
            drawer_content,
            class: String::default(),
            drawer: Direction::Bottom,
            hide_title: false,
            hide_close_x: false,
        }
    }
    pub(crate) fn get_options(self: &Self) -> AppDrawerOptions {
        let mut builder: AppDrawerOptionsBuilder =
            AppDrawerOptions::new(self.title.to_owned(), self.drawer_content);
        if self.hide_title {
            builder.hide_title();
        } else if self.hide_close_x {
            builder.hide_close_x();
        }

        builder.build()
    }
}

impl DrawerToggleInfoBuilder {
    pub fn build(self: &mut Self) -> DrawerToggleInfo {
        DrawerToggleInfo {
            display: self.display,
            title: self.title.to_string(),
            class: self.class.to_string(),
            drawer: self.drawer.to_owned(),
            drawer_content: self.drawer_content,
            hide_title: self.hide_title,
            hide_close_x: self.hide_close_x,
        }
    }
    pub fn set_drawer(self: &mut Self, drawer: Direction) -> &mut DrawerToggleInfoBuilder {
        self.drawer = drawer;
        self
    }

    pub fn hide_title(self: &mut Self) -> &mut DrawerToggleInfoBuilder {
        self.hide_title = true;
        self
    }
}
