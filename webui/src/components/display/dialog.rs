use crate::*;
use yew::Html;

pub struct Dialog {
    info: DrawerToggleInfoBuilder,
}
impl Dialog {
    pub fn new(title: &str, content: fn() -> Html) -> Self {
        Dialog {
            info: DrawerToggleInfo::new(title, || html!(), content)
                .set_drawer(Direction::Top)
                .set_on_confirm("Confirm", |_| true)
                .to_owned(),
        }
    }
    /// Create a new dialog without any close buttons, only a confirmation button.
    pub fn alert(title: &str, content: &dyn Fn() -> Html) -> Self {
		let test:fn()->Html = ||{html!{<>{"Mock Content"}</>}};
        Dialog {
            info: DrawerToggleInfo::new(title, || html!(), test)
                .set_drawer(Direction::Top)
                .hide_cancel_button()
                .hide_close_x_button()
                .set_on_confirm("Ok", |_| true)
                .set_content_class("auto-size")
                .to_owned(),
        }
    }
    pub fn message(&mut self) -> DrawerMessage {
        DrawerMessage::ToggleDrawer(self.info.build().get_options())
    }
    // pub fn open(&mut self) -> &Self {
    //     self.info.build().open();
    //     self
    // }
    pub fn auto_size(&mut self) -> &Self {
        self.info.set_content_class("auto-size");
        self
    }
    pub fn set_button_class(&mut self, class: &str) -> &mut Self {
        self.info.set_button_class(class);
        self
    }
    pub fn set_drawer(&mut self, drawer: Direction) -> &mut Self {
        self.info.set_drawer(drawer);
        self
    }

    pub fn hide_header(&mut self) -> &mut Self {
        self.info.hide_header();
        self
    }

    pub fn hide_footer(&mut self) -> &mut Self {
        self.info.hide_footer();
        self
    }

    pub fn hide_cancel_button(&mut self) -> &mut Self {
        self.info.hide_cancel_button();
        self
    }

    pub fn hide_close_x_button(&mut self) -> &mut Self {
        self.info.hide_close_x_button();
        self
    }

    /// Set the confirmation display text and handler handler
    ///
    /// This button will display on the right side of the drawer footer
    pub fn set_on_confirm(&mut self, display: &str, on_confirm: fn(Contexts) -> bool) -> &mut Self {
        self.info.set_on_confirm(display, on_confirm);
        self
    }
}
