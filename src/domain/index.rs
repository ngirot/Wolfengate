#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum TextureIndex {
    WALL,
    VOID,
    ENEMY,
    DOOR,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum FontIndex {
    MONTSERRAT,
}
