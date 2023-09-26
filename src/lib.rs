use std::{
	fs::{self, File},
	io,
	path::Path,
};

// TODO: Should BlurPlugin be Send?
pub trait BlurPlugin {
	/// The name of this plugin.
	fn name(&self) -> &'static str;

	/// Use this function to listen to game events.
	fn on_event(&self, event: &BlurEvent);

	/// When the game unloads this plugin.
	fn free(&self);
}

// TODO: Send + Sync ?
pub trait BlurAPI: Send + Sync {
	/// Set the target FPS for the frame limiter.
	/// Set to zero to disable FPS limit.
	fn set_fps(&mut self, fps: f64) -> bool;

	/// Get the frame limiter target FPS.
	/// Zero means uncapped.
	fn get_fps(&self) -> f64;

	// FIXME: How will this be used?
	#[doc(hidden)]
	fn register_event(&mut self, event: &BlurEvent);

	/// Notify the API of game events.
	/// Some notifications trigger [BlurPlugin::on_event].
	/// Careful with calling this from [BlurPlugin::on_event], don't create an infinite loop!
	fn notify(&self, notif: BlurNotification);
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

/// Convenience for creating log files next to the Blur savefile.
/// Usually this is at `"%APPDATA%\\bizarre creations\\blur\\amax\\log"`
pub fn create_log_file(name: impl AsRef<Path>) -> Result<File, io::Error> {
	let dir = known_folders::get_known_folder_path(known_folders::KnownFolder::RoamingAppData)
		.ok_or_else(|| io::Error::other("Couldn't get %APPDATA%/Roaming as a KnownFolder"))?
		.join("bizarre creations")
		.join("blur")
		.join("amax")
		.join("log");
	if !&dir.is_dir() {
		fs::create_dir_all(&dir)?;
	}
	let log_file = dir.join(name);
	File::create(log_file)
}


/// What the exported `plugin_init` function should look like:
///FIXME: `&'static` lifetime is a lie.
/// ```rust
/// #[no_mangle]
/// fn plugin_init(api: &'static mut dyn BlurAPI) -> Box<dyn BlurPlugin>;
/// ```
///TODO: Consider some sort of `blur_plugin_init!()` macro.
pub type FnPluginInit = fn(&mut dyn BlurAPI) -> Box<dyn BlurPlugin>;
