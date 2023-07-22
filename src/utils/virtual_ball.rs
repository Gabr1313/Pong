pub struct VirtualBall {
    x: i32,
    y: i32,
    step_x: i32,
    step_y: i32,
    indexes: Option<(usize, usize)>,
}

impl VirtualBall {
    pub fn new(x: i32, y: i32, step_x: i32, step_y: i32, indexes: Option<(usize, usize)>) -> Self {
        Self {
            x,
            y,
            step_x,
            step_y,
            indexes,
        }
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn step_x(&self) -> i32 {
        self.step_x
    }
    pub fn step_y(&self) -> i32 {
        self.step_y
    }
    pub fn indexes(&self) -> Option<(usize, usize)> {
        self.indexes
    }
}
