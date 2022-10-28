#[derive(Copy, Clone)]
pub struct ScreenPoint {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
pub struct MapPoint {
    x: u8,
    y: u8,
}
#[derive(Copy, Clone)]
pub struct Position {
    x: f32,
    y: f32,
}

impl ScreenPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

impl MapPoint {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn with_x(&self, x: f32) -> Self {
        Position { x, y: self.y }
    }

    pub fn with_y(&self, y: f32) -> Self {
        Position { x: self.x, y }
    }

    pub fn distance(&self, position: &Position) -> f32 {
        ((self.x - position.x).abs().powi(2) + (self.y - position.y).abs().powi(2)).sqrt()
    }
}
