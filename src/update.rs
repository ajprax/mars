use crate::app::AppEvent;


pub trait Update {
    fn update(&mut self) -> Option<AppEvent>;
}