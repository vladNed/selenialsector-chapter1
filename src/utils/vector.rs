use std::ops::AddAssign;

#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    x: f32,
    y: f32,
}
impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn as_vec(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}