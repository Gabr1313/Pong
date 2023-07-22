pub struct Segmet1D {
    x1: i32,
    x2: i32,
}

impl Segmet1D {
    pub fn new(x1: i32, x2: i32) -> Self {
        Self { x1, x2 }
    }

    pub fn intersect(&self, other: &Self) -> bool {
        lesseq_lesseq(self.x1, other.x1, self.x2)
            || lesseq_lesseq(self.x1, other.x2, self.x2)
            || lesseq_lesseq(other.x1, self.x1, other.x2)
            || lesseq_lesseq(other.x1, self.x2, other.x2)
    }
}

#[derive(Debug, Clone)]
pub struct Segmet2D {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Segmet2D {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    /// overlapping segments returns None
    pub fn intersect(&self, other: &Self) -> Option<(i32, i32)> {
        let alpha = self.y1 - self.y2;
        let beta = self.x2 - self.x1;
        let alpha_2 = other.y1 - other.y2;
        let beta_2 = other.x2 - other.x1;
        let denom = alpha * beta_2 - alpha_2 * beta;
        if denom == 0 {
            return None;
        }
        let gamma = self.x2 * (self.y1 - self.y2) - self.y2 * (self.x1 - self.x2);
        let gamma_2 = other.x2 * (other.y1 - other.y2) - other.y2 * (other.x1 - other.x2);
        let res_x = ((beta_2 * gamma) - (beta * gamma_2)) / denom;
        let res_y = ((alpha * gamma_2) - (alpha_2 * gamma)) / denom;

        if lesseq_lesseq(self.x1, res_x, self.x2)
            && lesseq_lesseq(other.x1, res_x, other.x2)
            && lesseq_lesseq(self.y1, res_y, self.y2)
            && lesseq_lesseq(other.y1, res_y, other.y2)
        {
            Some((res_x, res_y))
        } else {
            None
        }
    }
    pub fn change_x1(&mut self, new_x1: i32) {
        self.x1 = new_x1;
    }
    pub fn change_y1(&mut self, new_y1: i32) {
        self.y1 = new_y1;
    }
    pub fn change_x2(&mut self, new_x2: i32) {
        self.x2 = new_x2;
    }
    pub fn change_y2(&mut self, new_y2: i32) {
        self.y2 = new_y2;
    }
    pub fn y_min(&self) -> i32 {
        if self.y1 < self.y2 {
            self.y1
        } else {
            self.y2
        }
    }
    pub fn y_max(&self) -> i32 {
        if self.y1 > self.y2 {
            self.y1
        } else {
            self.y2
        }
    }
}

fn lesseq_lesseq<T: PartialOrd>(a: T, b: T, c: T) -> bool {
    (a <= b && b <= c) || (c <= b && b <= a)
}

#[test]
fn test() {
    let s1 = Segmet2D::new(1, 2, 1, 6);
    let s2 = Segmet2D::new(0, 4, 2, 4);
    assert_eq!(s1.intersect(&s2), Some((1, 4)));
    let s1 = Segmet2D::new(316, 2, 316, -2);
    let s2 = Segmet2D::new(0, 0, 640, 0);
    assert_eq!(s1.intersect(&s2), Some((316, 0)));
}
