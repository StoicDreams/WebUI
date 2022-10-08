/// Macro for expanding string arguments into a title
///
/// This title will use the .theme-title class, which will match the color with the app header bar.
///
/// example
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		{title_standard!("Text for your title")}
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! title_standard {
	( $($x:expr ),* ) => {
		html! {
			<h2 class="theme-title pa-1 f4">
				$(
					{$x}
				)*
			</h2>
		}
	};
}

/// Macro for expanding string arguments into a title
///
/// This title will use the .theme-active class.
///
/// example
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		{title_active!("Text for your title")}
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! title_active {
	( $($x:expr ),* ) => {
		html! {
			<h2 class="theme-active pa-1 f4">
				$(
					{$x}
				)*
			</h2>
		}
	};
}

/// Macro for expanding string arguments into a title
///
/// This title will use the .theme-background class.
///
/// example
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		{title_background!("Text for your title")}
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! title_background {
	( $($x:expr ),* ) => {
		html! {
			<h2 class="theme-background pa-1 f4">
				$(
					{$x}
				)*
			</h2>
		}
	};
}

/// Macro for expanding string arguments into a title
///
/// This title will use the .theme-black class.
///
/// example
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		{title_black!("Text for your title")}
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! title_black {
	( $($x:expr ),* ) => {
		html! {
			<h2 class="theme-black pa-1 f4">
				$(
					{$x}
				)*
			</h2>
		}
	};
}

/// Macro for expanding string arguments into a title
///
/// This title will use the .theme-white class.
///
/// example
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		{title_white!("Text for your title")}
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! title_white {
	( $($x:expr ),* ) => {
		html! {
			<h2 class="theme-white pa-1 f4">
				$(
					{$x}
				)*
			</h2>
		}
	};
}

/// Macro for expanding string arguments into a title
///
/// This title will use the .theme-primary class.
///
/// example
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		{title_primary!("Text for your title")}
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! title_primary {
	( $($x:expr ),* ) => {
		html! {
			<h2 class="theme-primary pa-1 f4">
				$(
					{$x}
				)*
			</h2>
		}
	};
}

/// Macro for expanding string arguments into a title
///
/// This title will use the .theme-secondary class.
///
/// example
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		{title_secondary!("Text for your title")}
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! title_secondary {
	( $($x:expr ),* ) => {
		html! {
			<h2 class="theme-secondary pa-1 f4">
				$(
					{$x}
				)*
			</h2>
		}
	};
}

/// Macro for expanding string arguments into a title
///
/// This title will use the .theme-tertiary class.
///
/// example
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		{title_tertiary!("Text for your title")}
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! title_tertiary {
	( $($x:expr ),* ) => {
		html! {
			<h2 class="theme-tertiary pa-1 f4">
				$(
					{$x}
				)*
			</h2>
		}
	};
}

/// Macro for expanding string arguments into a title
///
/// This title will use the .theme-info class.
///
/// example
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		{title_info!("Text for your title")}
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! title_info {
	( $($x:expr ),* ) => {
		html! {
			<h2 class="theme-info pa-1 f4">
				$(
					{$x}
				)*
			</h2>
		}
	};
}

/// Macro for expanding string arguments into a title
///
/// This title will use the .theme-success class.
///
/// example
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		{title_success!("Text for your title")}
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! title_success {
	( $($x:expr ),* ) => {
		html! {
			<h2 class="theme-success pa-1 f4">
				$(
					{$x}
				)*
			</h2>
		}
	};
}

/// Macro for expanding string arguments into a title
///
/// This title will use the .theme-warning class.
///
/// example
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		{title_warning!("Text for your title")}
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! title_warning {
	( $($x:expr ),* ) => {
		html! {
			<h2 class="theme-warning pa-1 f4">
				$(
					{$x}
				)*
			</h2>
		}
	};
}

/// Macro for expanding string arguments into a title
///
/// This title will use the .theme-danger class.
///
/// example
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		{title_danger!("Text for your title")}
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! title_danger {
	( $($x:expr ),* ) => {
		html! {
			<h2 class="theme-danger pa-1 f4">
				$(
					{$x}
				)*
			</h2>
		}
	};
}
