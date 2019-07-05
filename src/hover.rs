use crate::app::AppEvent;

/// trait for types which should receive cursor move events
pub trait Hover {
    fn hover(&mut self, window_size: [f64; 2], cursor: [f64; 2]) -> Option<AppEvent>;
}