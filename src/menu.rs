use std::collections::HashMap;

use graphics::character::CharacterCache;
use graphics::clear;
use graphics::Context;
use graphics::ellipse;
use graphics::image;
use graphics::text;
use graphics::Transformed;
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use opengl_graphics::GlyphCache;
use opengl_graphics::Texture;
use piston::input::Button;
use piston::input::Key;
use piston::input::MouseButton;

use crate::app::AppEvent;
use crate::app::MissionDifficulty;
use crate::colors;
use crate::fonts;
use crate::hover::Hover;
use crate::images;
use crate::press::Press;
use crate::release::Release;
use crate::render::Render;
use crate::sounds::play_sound;
use crate::sounds::Sound;


const MENU_BUTTON_FONT_SIZE: u32 = 32;


pub struct MenuButton {
    text: &'static str,
    pos: [f64; 2],
    hovered: bool,
}

impl MenuButton {
    pub fn new(text: &'static str, pos: [f64; 2], hovered: bool) -> MenuButton {
        MenuButton { text, pos, hovered }
    }

    pub fn contains_point(&self, window_size: [f64; 2], point: [f64; 2]) -> bool {
        let [w, h] = window_size;
        let [dx, dy] = self.pos;
        let [x, y] = [w * 0.1 + dx, w * 0.1 + dy];
        let [px, py] = point;
        let mut font = fonts::FONT.lock().unwrap();
        x < px && px < x + font.width(MENU_BUTTON_FONT_SIZE, self.text).unwrap() && (y - MENU_BUTTON_FONT_SIZE as f64) < py && py < y
    }
}

impl Render for MenuButton {
    fn render(&self, c: &Context, gl: &mut GlGraphics) {
        let [w, h] = c.get_view_size();
        let [dx, dy] = self.pos;
        let [x, y] = [w * 0.1 + dx, w * 0.1 + dy];
        let mut font = fonts::FONT.lock().unwrap();
        text(
            if self.hovered {
                *colors::MARS
            } else {
                *colors::RED
            },
            MENU_BUTTON_FONT_SIZE,
            self.text,
            &mut *font,
            c.transform.trans(x, y),
            gl
        );
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SubMenu {
    NewMission,
    LoadMission,
    Options,
}

pub struct MenuState {
    /// whether there's a mission running which can be resumed
    pub active_mission: bool,
    /// whether and which submenus is shown
    pub submenu: Option<SubMenu>,
    /// buttons on the current menu, swapped whenever submenu changes
    pub buttons: Vec<MenuButton>,
    /// index of the hovered button (if any), used for detecting presses
    pub hovered: Option<usize>,
    /// indicates that the left mouse button went down while hovering the button at this index. only
    /// consider a button pressed if the cursor is there on both down and up
    pub pressing: Option<usize>,
}

impl MenuState {
    fn main_menu_buttons(active_mission: bool) -> Vec<MenuButton> {
        let mut buttons = Vec::new();
        let mut y = 0.0;
        if active_mission {
            buttons.push(MenuButton::new("resume", [0.0, y], false));
            y += 50.0;
        }
        buttons.push(MenuButton::new("new mission", [0.0, y], false));
        y += 50.0;
        buttons.push(MenuButton::new("load mission", [0.0, y], false));
        y += 50.0;
        buttons.push(MenuButton::new("options", [0.0, y], false));
        y += 50.0;
        buttons.push(MenuButton::new("exit", [0.0, y], false));
        buttons
    }

    fn new_mission_menu_buttons() -> Vec<MenuButton> {
        let mut buttons = Vec::new();
        let mut y = 0.0;
        buttons.push(MenuButton::new("easy", [0.0, y], false));
        y += 50.0;
        buttons.push(MenuButton::new("medium", [0.0, y], false));
        y += 50.0;
        buttons.push(MenuButton::new("hard", [0.0, y], false));
        y += 100.0;
        buttons.push(MenuButton::new("back", [0.0, y], false));
        buttons
    }

    fn load_mission_menu_buttons() -> Vec<MenuButton> {
        let mut buttons = Vec::new();
        // TODO
        buttons
    }

    fn options_menu_buttons() -> Vec<MenuButton> {
        let mut buttons = Vec::new();
        // TODO
        buttons
    }

    pub fn new(active_mission: bool) -> MenuState {
        MenuState {
            active_mission,
            submenu: None,
            buttons: MenuState::main_menu_buttons(active_mission),
            hovered: None,
            pressing: None,
        }
    }
}

impl Render for MenuState {
    fn render(
        &self,
        c: &Context,
        gl: &mut GlGraphics
    ) {
        clear(*colors::BLACK, gl);
        let [w, h] = c.get_view_size();

        // title and background
        image(&*images::MARS, c.transform.trans((w * 0.4).floor(), h * 0.1).scale(w / 2560.0, w / 2560.0), gl);
        {
            let mut title_font = fonts::TITLE_FONT.lock().unwrap();
            text(*colors::RED, (h * 0.3) as u32, "MARS", &mut *title_font, c.transform.trans(w * 0.1, h - w * 0.1), gl).unwrap();
        }

        // menu options
        for button in self.buttons.iter() {
            button.render(c, gl);
        }
    }
}

impl Hover for MenuState {
    fn hover(&mut self, window_size: [f64; 2], cursor: [f64; 2]) -> Option<AppEvent> {
        let previously_hovered = self.hovered;
        // unset hovered and then reset it if we're still hovering, this way if we're not hovering
        // it gets unset correctly
        self.hovered = None;
        for (i, button) in self.buttons.iter_mut().enumerate() {
            if button.contains_point(window_size, cursor) {
                button.hovered = true;
                if previously_hovered != Some(i) {
                    play_sound(Sound::TapMuted, 0.5);
                }
                self.hovered = Some(i);
            } else {
                button.hovered = false;
            }
        }
        None
    }
}

impl Press for MenuState {
    fn press(&mut self, button: Button) -> Option<AppEvent> {
        self.pressing = self.hovered;
        None
    }
}

impl Release for MenuState {
    fn release(&mut self, button: Button) -> Option<AppEvent>  {
        if let Button::Mouse(MouseButton::Left) = button {
            let pressing = self.pressing;
            self.pressing = None;
            if pressing == self.hovered {
                if let Some(i) = pressing {
                    match (self.submenu, self.active_mission, i) {
                        // main menu
                        (None, true, 0) => return Some(AppEvent::ResumeMission),
                        (None, true, 1) | (None, false, 0) => {
                            self.submenu = Some(SubMenu::NewMission);
                            self.buttons = MenuState::new_mission_menu_buttons();
                        },
                        (None, true, 2) | (None, false, 1) => {
                            self.submenu = Some(SubMenu::LoadMission);
                            self.buttons = MenuState::load_mission_menu_buttons();
                        },
                        (None, true, 3) | (None, false, 2) => {
                            self.submenu = Some(SubMenu::Options);
                            self.buttons = MenuState::options_menu_buttons();
                        },
                        (None, true, 4) | (None, false, 3) => return Some(AppEvent::Exit),
                        // new mission menu
                        (Some(SubMenu::NewMission), _, 0) => return Some(AppEvent::NewMission(MissionDifficulty::Easy)),
                        (Some(SubMenu::NewMission), _, 1) => return Some(AppEvent::NewMission(MissionDifficulty::Medium)),
                        (Some(SubMenu::NewMission), _, 2) => return Some(AppEvent::NewMission(MissionDifficulty::Hard)),
                        (Some(SubMenu::NewMission), am, 3) => {
                            self.submenu = None;
                            self.buttons = MenuState::main_menu_buttons(am);
                        },
                        // load mission menu
                        // TODO
                        // options menu
                        (Some(SubMenu::Options), _, i) => println!("button {} pressed in options menu", i),
                        // TODO
                        _ => {},
                    }
                }
            }
        } else if let Button::Keyboard(Key::Escape) = button {
            if self.submenu.is_some() {
                self.submenu = None;
                self.buttons = MenuState::main_menu_buttons(self.active_mission);
                return None
            } else {
                return Some(AppEvent::ResumeMission)
            }
        }
        None
    }
}
