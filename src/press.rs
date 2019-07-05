use piston::input::Button;

use crate::app::AppEvent;


/// trait for types which can respond to keyboard and mouse button down events
pub trait Press {
    fn press(&mut self, button: Button) -> Option<AppEvent>;
}