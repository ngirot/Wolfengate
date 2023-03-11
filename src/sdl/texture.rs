use std::collections::HashMap;

use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use crate::domain::index::{FontIndex, TextureIndex};

pub struct ResourceRegistry<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    ttf_context: &'a Sdl2TtfContext,

    texture_registry: HashMap<TextureIndex, LoadedTexture<'a>>,
    font_registry: HashMap<FontIndex, Font<'a, 'a>>,
}

pub struct LoadedTexture<'a> {
    data: Texture<'a>,
    width: u32,
    height: u32,
}

impl<'s> ResourceRegistry<'s> {
    pub fn new(
        texture_creator: &'s TextureCreator<WindowContext>,
        ttf_creator: &'s Sdl2TtfContext,
    ) -> ResourceRegistry<'s> {
        Self {
            texture_creator,
            ttf_context: ttf_creator,
            texture_registry: HashMap::new(),
            font_registry: HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, index: TextureIndex, file: String) {
        let texture = load_texture(self.texture_creator, file);
        let query = texture.query();

        let loaded_texture = LoadedTexture::new(texture, query.width, query.height);

        self.texture_registry.insert(index, loaded_texture);
    }

    pub fn load_font(&mut self, index: FontIndex, filename: String) {
        let path = build_path(filename);
        let font = self.ttf_context.load_font(path, 128).unwrap();

        self.font_registry.insert(index, font);
    }

    pub fn get_texture(&self, index: TextureIndex) -> Option<&LoadedTexture> {
        self.texture_registry.get(&index)
    }

    pub fn get_font(&self, index: FontIndex) -> Option<&Font> {
        self.font_registry.get(&index)
    }
}

impl<'s> LoadedTexture<'s> {
    fn new(data: Texture, width: u32, height: u32) -> LoadedTexture {
        LoadedTexture {
            data,
            width,
            height,
        }
    }

    pub fn data(&self) -> &Texture {
        &self.data
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

fn load_texture(texture_creator: &TextureCreator<WindowContext>, filename: String) -> Texture {
    let path = build_path(filename);
    let texture = texture_creator
        .load_texture(path)
        .expect("Unable to load texture");
    texture
}

fn build_path(filename: String) -> String {
    let mut path = String::from("res/");
    path.push_str(&filename);

    path
}
