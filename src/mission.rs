use std::collections::HashMap;

use graphics::clear;
use graphics::Context;
use graphics::text;
use graphics::Transformed;
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use opengl_graphics::GlyphCache;
use opengl_graphics::Texture;
use piston::input::Button;
use piston::input::Key;

use crate::app::AppEvent;
use crate::colors;
use crate::fonts;
use crate::hover::Hover;
use crate::press::Press;
use crate::release::Release;
use crate::render::Render;
use crate::update::Update;


pub enum MissionState {
    Planning {
        budget: f32,
    },
    Execution {

    },
}

impl Hover for MissionState {
    fn hover(&mut self, window_size: [f64; 2], cursor: [f64; 2]) -> Option<AppEvent> {
        None
    }
}

impl Press for MissionState {
    fn press(&mut self, button: Button) -> Option<AppEvent> {
        println!("mission registered press of {:?}", button);
        None
    }
}

impl Release for MissionState {
    fn release(&mut self, button: Button) -> Option<AppEvent> {
        if let Button::Keyboard(Key::Escape) = button {
            Some(AppEvent::OpenMenu)
        } else {
            None
        }
    }
}

impl Render for MissionState {
    fn render(
        &self,
        c: &Context,
        gl: &mut GlGraphics
    ) {
        let [w, h] = c.get_view_size();
        clear(*colors::BLACK, gl);
        let mut font = fonts::FONT.lock().unwrap();

        match self {
            MissionState::Planning { budget } => {
                text(*colors::RED, 32, &format!("Remaining Budget: ${:?} (millions)", budget), &mut *font, c.transform.trans(w * 0.1, h - w * 0.1), gl).unwrap();
            },
            _ => text(*colors::RED, 32, "under construction", &mut *font, c.transform.trans(w * 0.1, h - w * 0.1), gl).unwrap(),
        }

    }
}

impl Update for MissionState {
    fn update(&mut self) -> Option<AppEvent> {
        None
    }
}