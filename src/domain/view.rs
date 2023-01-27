#[derive(Copy, Clone)]
pub struct ViewScreen {
    height: u16,
    width: u16,
    angle: f32,
}

impl ViewScreen {
    pub fn new(height: u16, width: u16, angle: f32) -> Self {
        Self { height, width, angle }
    }

    pub fn height(&self) -> i32 {
        self.height as i32
    }
    pub fn width(&self) -> i32 {
        self.width as i32
    }
    pub fn angle(&self) -> f32 {
        self.angle
    }
}