use graphics::Context;
use glutin_window::GlutinWindow;
use opengl_graphics::GlGraphics;
use piston::input::Button;
use piston::input::keyboard::Key;
use piston::window::Window;
use piston::window::Size as WindowSize;

use crate::hover::Hover;
use crate::menu::MenuState;
use crate::mission::MissionState;
use crate::press::Press;
use crate::release::Release;
use crate::render::Render;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionDifficulty {
    Easy,
    Medium,
    Hard,
}


/// returned from various event handlers to allow lower level components to affect app level state
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppEvent {
    Exit,
    OpenMenu,
    ResumeMission,
    NewMission(MissionDifficulty),
}


pub struct AppState {
    pub window: GlutinWindow,
    pub menu_state: Option<MenuState>,
    pub mission_state: Option<MissionState>,
}

impl AppState {
    pub fn new(window: GlutinWindow) -> AppState {
        AppState {
            window,
            menu_state: Some(MenuState::new(false)),
            mission_state: None,
        }
    }

    pub fn handle(&mut self, event: AppEvent) {
        match event {
            AppEvent::Exit => self.window.set_should_close(true),
            AppEvent::OpenMenu => self.menu_state = Some(MenuState::new(self.mission_state.is_some())),
            AppEvent::ResumeMission => if self.mission_state.is_some() { self.menu_state = None },
            AppEvent::NewMission(difficulty) => {
                self.mission_state = Some(MissionState::Planning {
                    budget: match difficulty {
                        MissionDifficulty::Easy => 1000.0,
                        MissionDifficulty::Medium => 750.0,
                        MissionDifficulty::Hard => 500.0,
                    }
                });
                self.menu_state = None;
            }
        }
    }

    pub fn window_size(&self) -> [f64; 2] {
        let WindowSize { width, height } = self.window.size();
        [width, height]
    }
}

impl Hover for AppState {
    fn hover(&mut self, window_size: [f64; 2], cursor: [f64; 2]) -> Option<AppEvent> {
        match (&mut self.menu_state, &mut self.mission_state) {
            (Some(ref mut menu_state), Some(_)) | (Some(ref mut menu_state), None) => menu_state.hover(window_size, cursor),
            (None, Some(ref mut mission_state)) => mission_state.hover(window_size, cursor),
            (None, None) => None
        }
    }
}

impl Press for AppState {
    fn press(&mut self, button: Button) -> Option<AppEvent> {
        if let Button::Keyboard(Key::Escape) = button {
            None
        } else {
            match (&mut self.menu_state, &mut self.mission_state) {
                (Some(ref mut menu_state), Some(_)) | (Some(ref mut menu_state), None) => menu_state.press(button),
                (None, Some(ref mut mission_state)) => mission_state.press(button),
                (None, None) => None
            }
        }
    }
}

impl Release for AppState {
    fn release(&mut self, button: Button) -> Option<AppEvent> {
        match (&mut self.menu_state, &mut self.mission_state) {
            (Some(ref mut menu_state), Some(_)) | (Some(ref mut menu_state), None) => menu_state.release(button),
            (None, Some(ref mut mission_state)) => mission_state.release(button),
            (None, None) => None
        }
    }
}

impl Render for AppState {
    fn render(&self, c: &Context, gl: &mut GlGraphics) {
        if let Some(ref menu_state) = self.menu_state {
            menu_state.render(&c, gl);
        } else if let Some(ref mission_state) = self.mission_state {
            mission_state.render(&c, gl);
        }
    }
}