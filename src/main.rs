extern crate glutin_window;
extern crate graphics;
#[macro_use]
extern crate lazy_static;
extern crate opengl_graphics;
extern crate piston;
extern crate rayon;

use glutin_window::GlutinWindow;
use graphics::Context;
use opengl_graphics::OpenGL;
use opengl_graphics::GlGraphics;
use piston::event_loop::Events;
use piston::event_loop::EventSettings;
use piston::input::MouseCursorEvent;
use piston::input::PressEvent;
use piston::input::ReleaseEvent;
use piston::input::ResizeEvent;
use piston::input::RenderEvent;
use piston::window::AdvancedWindow;
use piston::window::Window;
use piston::window::WindowSettings;
use piston::window::Size as WindowSize;

mod app;
mod colors;
mod fonts;
mod hover;
mod images;
mod menu;
mod mission;
mod press;
mod release;
mod render;
mod sounds;
mod update;

use app::AppState;
use hover::Hover;
use press::Press;
use release::Release;
use render::Render;
use sounds::play_sound;
use sounds::Sound;


// TODO: model state changes through events. so the collision detector would just emit a
// Collision(A, B) event and both the list of planets and the focused state could update as a result
// these events can look basically the same as the update/render/input events in the base event loop
// every event is dispatched to each subsystem, they're all allowed to update in response

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: GlutinWindow = WindowSettings::new("mars", [1, 1])
        .graphics_api(opengl)
        .fullscreen(true)
        .build()
        .expect("failed to create window");

    let mut gl = GlGraphics::new(opengl);

    let mut event_settings = EventSettings::new();
    event_settings.max_fps = 140;
    event_settings.ups = 240;
    let mut events = Events::new(event_settings);

    let mut app = AppState::new(window);

    use std::io::BufReader;
    use std::fs::File;

    play_sound(Sound::Holst, 0.5);

    while let Some(e) = events.next(&mut app.window) {
        // we don't know what size the screen is, but with fullscreen(true) we'll get a resize event
        // that sets us to the full size of the screen before getting an event that sets us to the
        // configured size. we take advantage of that event to set the size correctly.
        e.resize(|r| {
            let [ow, oh] = app.window_size();
            let [w, h] = r.window_size;
            if w > ow || h > oh {
                app.window.set_size([w, h]);
            }
        });

        e.mouse_cursor(|c| {
            let ae = app.hover(app.window_size(), c);
            if let Some(app_event) = ae { app.handle(app_event) }
        });

        e.press(|b| {
            let ae = app.press(b);
            if let Some(app_event) = ae { app.handle(app_event) }
        });

        e.release(|b| {
            let ae = app.release(b);
            if let Some(app_event) = ae { app.handle(app_event) }
        });

        e.render(|r| {
            gl.draw(r.viewport(), |c: Context, gl: &mut GlGraphics| {
                app.render(&c, gl);
            });
        });
    }
}
