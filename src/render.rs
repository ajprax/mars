use std::collections::HashMap;

use graphics::Context;
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use opengl_graphics::GlyphCache;
use opengl_graphics::Texture;


pub trait Render {
    fn render(
        &self,
        c: &Context,
        gl: &mut GlGraphics
    );
}