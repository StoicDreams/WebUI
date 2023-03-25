use crate::*;

/// Struct used for defining details for displaying buttons that toggle drawer content.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawerToggleInfo {
    pub(crate) display: fn() -> Html,
    pub(crate) title: String,
    pub(crate) class: String,
    pub(crate) drawer: Direction,
    pub(crate) drawer_content: fn() -> Html,
    pub(crate) hide_header: bool,
    pub(crate) hide_footer: bool,
    pub(crate) hide_close_x: bool,
    pub(crate) hide_cancel: bool,
    pub(crate) on_confirm: Option<fn() -> bool>,
    pub(crate) confirm_display: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DrawerToggleInfoBuilder {
    display: fn() -> Html,
    title: String,
    class: String,
    drawer: Direction,
    drawer_content: fn() -> Html,
    hide_header: bool,
    hide_footer: bool,
    hide_close_x: bool,
    hide_cancel: bool,
    on_confirm: Option<fn() -> bool>,
    confirm_display: String,
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
            hide_header: false,
            hide_footer: false,
            hide_close_x: false,
            hide_cancel: false,
            on_confirm: None,
            confirm_display: String::default(),
        }
    }
    pub(crate) fn get_options(self: &Self) -> AppDrawerOptions {
        let mut builder: AppDrawerOptionsBuilder =
            AppDrawerOptions::new(self.title.to_owned(), self.drawer_content);
        if self.hide_header {
            builder.hide_header();
        } else if self.hide_close_x {
            builder.hide_close_x();
        }
        if self.hide_footer {
            builder.hide_footer();
        } else {
            if self.hide_cancel {
                builder.hide_cancel();
            }
            match self.on_confirm {
                Some(on_confirm) => {
                    builder.set_on_confirm(self.confirm_display.to_string(), on_confirm);
                }
                None => (),
            }
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
            hide_header: self.hide_header,
            hide_footer: self.hide_footer,
            hide_close_x: self.hide_close_x,
            hide_cancel: self.hide_cancel,
            on_confirm: self.on_confirm,
            confirm_display: self.confirm_display.to_string(),
        }
    }
    pub fn set_button_class(self: &mut Self, class: String) -> &mut Self {
        self.class = class;
        self
    }
    pub fn set_drawer(self: &mut Self, drawer: Direction) -> &mut Self {
        self.drawer = drawer;
        self
    }

    pub fn hide_header(self: &mut Self) -> &mut Self {
        self.hide_header = true;
        self
    }

    pub fn hide_footer(self: &mut Self) -> &mut Self {
        self.hide_footer = true;
        self
    }

    pub fn hide_cancel_button(self: &mut Self) -> &mut Self {
        self.hide_cancel = true;
        self
    }

    /// Set the confirmation display text and handler handler
    ///
    /// This button will display on the right side of the drawer footer
    pub fn set_on_confirm(self: &mut Self, display: String, on_confirm: fn() -> bool) -> &mut Self {
        self.on_confirm = Some(on_confirm);
        self.confirm_display = display;
        self
    }
}
