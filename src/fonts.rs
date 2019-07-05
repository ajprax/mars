use std::sync::Mutex;

use opengl_graphics::GlyphCache;
use opengl_graphics::TextureSettings;


lazy_static! {
    pub static ref FONT: Mutex<GlyphCache<'static>> = Mutex::new(GlyphCache::new(find_folder::Search::ParentsThenKids(3, 3).for_folder("fonts").unwrap().join("Roboto-Regular.ttf"), (), TextureSettings::new()).unwrap());
    pub static ref TITLE_FONT: Mutex<GlyphCache<'static>> = Mutex::new(GlyphCache::new(find_folder::Search::ParentsThenKids(3, 3).for_folder("fonts").unwrap().join("House On Mars.ttf"), (), TextureSettings::new()).unwrap());
}
