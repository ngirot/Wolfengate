use std::collections::HashMap;

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use crate::domain::texture::TextureIndex;

pub struct TextureRegistry<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    registry: HashMap<TextureIndex, LoadedTexture<'a>>,
}

pub struct LoadedTexture<'a> {
    data: Texture<'a>,
    width: u32,
    height: u32,
}

impl<'s> TextureRegistry<'s> {
    pub fn new<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> TextureRegistry {
        let map = HashMap::new();

        TextureRegistry {
            texture_creator: &texture_creator,
            registry: map,
        }
    }

    pub fn load(&mut self, index: TextureIndex, file: String) {
        let texture = load_texture(self.texture_creator, file);
        let query = texture.query();

        let loaded_texture = LoadedTexture::new(texture, query.width, query.height);

        self.registry.insert(index, loaded_texture);
    }

    pub fn get(&self, index: TextureIndex) -> Option<&LoadedTexture> {
        self.registry.get(&index)
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
    let mut path = String::from("res/");
    path.push_str(&filename);
    let texture = texture_creator
        .load_texture(path)
        .expect("Unable to load texture");
    texture
}
