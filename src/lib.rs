pub trait BlurPlugin {
	fn name(&self) -> &'static str;
	fn on_event(&self, event: &BlurEvent);
	fn free(&self);
}

pub trait BlurAPI {
	fn set_fps(&mut self, fps: f64) -> bool;
	fn get_fps(&self) -> f64;
	fn register_event(&mut self, event: &BlurEvent);
	fn notify(&self, event: BlurEvent);
}

/// Game events used by the [BlurAPI].
/// They are sent when [BlurAPI::notify] is fired.
/// A [BlurPlugin] can listen to the events with [BlurPlugin::on_event]
#[derive(Debug)]
pub enum BlurEvent {
	/// Placeholder, don't know what I'll use this for yet
	/// TODO: ?
	NoEvent,
	/// Fired when the player presses the "Log in" button on entering online mode.
	LoginStart {
		/// username they tried to log in with
		username: String,
	},
	/// Fired after login, succesful or not
	LoginEnd {
		/// username they tried to log in with
		username: String,
		/// `true` if the login was succesful
		success: bool,
	},

	/// Fired on GoScreen(screen)
	/// TODO: docs + ?
	Screen {
		/// The name of the screen
		name: String,
	},
}

/// What the plugin_init function should look like:
/// ```rust
/// #[no_mangle]
/// fn plugin_init(api: &'static mut dyn BlurAPI) -> Box<dyn BlurPlugin>;
/// ```
pub type FnInit = fn(&mut dyn BlurAPI) -> Box<dyn BlurPlugin>;
