pub trait BlurPlugin {
	/// The name of this plugin
	fn name(&self) -> &'static str;

	/// Use this function to listen to game events
	fn on_event(&self, event: &BlurEvent);

	/// Run when the game unloads the plugin DLL
	fn free(&self);
}

pub trait BlurAPI {
	fn set_fps(&mut self, fps: f64) -> bool;
	fn get_fps(&self) -> f64;

	/// FIXME: How will this be used?
	fn register_event(&mut self, event: &BlurEvent);

	fn notify(&self, event: BlurNotification);
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

/// Used to notify the [BlurAPI] of game events, with [BlurAPI::notify]
/// A [BlurPlugin] can listen to the generated [BlurEvent] using [BlurPlugin::on_event]
#[derive(Debug)]
pub enum BlurNotification {
	/// TODO: ?
	Nothing,
	/// Player presses the "Log in" button to enter online mode.
	LoginStart,
	/// TODO: Post login, succesful or not
	LoginEnd { success: bool },

	/// TODO: ?
	Screen {
		/// The name of the screen
		name: String,
	},
}

/// What the plugin_init function should look like. * Careful with the `&'static` lifetime:
/// ```rust
/// #[no_mangle]
/// fn plugin_init(api: &'static mut dyn BlurAPI) -> Box<dyn BlurPlugin>;
/// ```
pub type FnPluginInit = fn(&mut dyn BlurAPI) -> Box<dyn BlurPlugin>;
