#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct TextureIndex {
    id: u128,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct FontIndex {
    id: u128,
}

impl TextureIndex {
    pub fn new(id: u128) -> Self {
        Self {
            id
        }
    }

    pub fn id(&self) -> u128 {
        self.id
    }
}

impl FontIndex {
    pub fn new(id: u128) -> Self {
        Self {
            id
        }
    }

    pub fn id(&self) -> u128 {
        self.id
    }
}