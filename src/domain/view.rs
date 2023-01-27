use std::f32::consts::PI;

#[derive(Copy, Clone)]
pub struct ViewScreen {
    height: u16,
    width: u16,
    angle: f32,
    ratio: f32,
}

impl ViewScreen {
    pub fn new(height: u16, width: u16) -> Self {
        Self { height, width, angle: PI / 2.0, ratio: 0.8 }
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

    pub fn ratio(&self) -> f32 {
        self.ratio
    }
}