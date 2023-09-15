pub trait BlurPlugin {
	fn name(&self) -> &'static str;
	fn on_event(&self, event: &BlurEvent);
	fn free(&self);
}

pub trait BlurAPI {
	fn set_fps(&mut self, fps: f64) -> bool;
	fn get_fps(&self) -> f64;
	fn register_event(&mut self, event: &BlurEvent);
	fn notify(&self, event: &BlurEvent);
}

#[derive(Debug)]
pub enum BlurEvent {
	NoEvent,
	Login(u32),
	Screen(u32),
}

pub type FnInit = fn(&mut dyn BlurAPI) -> Box<dyn BlurPlugin>;
