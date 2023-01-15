#[derive(Copy, Clone)]
pub struct Force {
    orientation: f32,
    power: f32,
    rotation: f32,
}

impl Force {
    pub fn new(orientation: f32, power: f32, rotation: f32) -> Self {
        Force { orientation, power, rotation }
    }

    pub fn orientation(&self) -> f32 {
        self.orientation
    }

    pub fn power(&self) -> f32 {
        self.power
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }
}