use opengl_graphics::Texture;
use opengl_graphics::TextureSettings;
use rayon::prelude::*;


lazy_static! {
    pub static ref MARS: Texture = Texture::from_path(find_folder::Search::ParentsThenKids(3, 3).for_folder("images").unwrap().join("mars.png"), &TextureSettings::new()).unwrap();
}
