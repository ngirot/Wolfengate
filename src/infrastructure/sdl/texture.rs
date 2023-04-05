use std::collections::HashMap;

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use sdl2::ttf::{Font, Sdl2TtfContext};

use crate::domain::resources::ResourceLoader;
use crate::domain::topology::index::{FontIndex, TextureIndex};

pub trait ResourceRegistryLoader {
    fn load_texture(&mut self, file: String) -> TextureIndex;
    fn load_font(&mut self, filename: String) -> FontIndex;
}

pub struct ResourceRegistry<'a> {
    id: u128,
    texture_creator: &'a TextureCreator<WindowContext>,
    ttf_context: &'a Sdl2TtfContext,
    resource_loader: &'a ResourceLoader,

    texture_registry: HashMap<u128, LoadedTexture<'a>>,
    font_registry: HashMap<u128, Font<'a, 'a>>,
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
        resource_loader: &'s ResourceLoader,
    ) -> ResourceRegistry<'s> {
        Self {
            id: 0,
            texture_creator,
            resource_loader,
            ttf_context: ttf_creator,
            texture_registry: HashMap::new(),
            font_registry: HashMap::new(),
        }
    }

    pub fn get_texture(&self, index: TextureIndex) -> Option<&LoadedTexture<'s>> {
        self.texture_registry.get(&index.id())
    }

    pub fn get_font(&self, index: FontIndex) -> Option<&Font<'s, 's>> {
        self.font_registry.get(&index.id())
    }

    fn generate_id(&mut self) -> u128 {
        let generated = self.id;
        self.id += 1;

        generated
    }
}

impl<'s> ResourceRegistryLoader for ResourceRegistry<'s> {
    fn load_texture(&mut self, file: String) -> TextureIndex {
        let texture = load_texture(self.texture_creator, file, self.resource_loader);
        let query = texture.query();

        let loaded_texture = LoadedTexture::new(texture, query.width, query.height);

        let current_id = self.generate_id();
        self.texture_registry.insert(current_id, loaded_texture);

        TextureIndex::new(current_id)
    }

    fn load_font(&mut self, filename: String) -> FontIndex {
        let file = self.resource_loader.load_as_file(filename);
        let font = self.ttf_context.load_font(file, 128).unwrap();
        let current_id = self.generate_id();
        self.font_registry.insert(current_id, font);

        FontIndex::new(current_id)
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

fn load_texture<'a>(texture_creator: &'a TextureCreator<WindowContext>, filename: String, resource_loader: &'a ResourceLoader) -> Texture<'a> {
    let path = resource_loader.load_as_binary(filename);
    let texture = texture_creator
        .load_texture_bytes(path.as_slice())
        .expect("Unable to load texture");
    texture
}