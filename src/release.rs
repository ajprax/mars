use piston::input::Button;

use crate::app::AppEvent;


/// trait for types which can respond to keyboard and mouse button up events
pub trait Release {
    fn release(&mut self, button: Button) -> Option<AppEvent>;
}