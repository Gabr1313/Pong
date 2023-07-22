use crate::history::History;
use crate::paddle::Paddle;
use crate::segment::Segmet2D;
use crate::team::TeamName;
use crate::virtual_ball::VirtualBall;
use rand::Rng;
use sdl2::rect::Rect;

pub struct Ball {
    rect: Rect,
    vx: i32,
    vy: i32,
    rect_default: Rect,
    vx_default: i32,
    vy_default: i32,
    multplier_max: f32,
    slow_start: f32,
}

impl Ball {
    pub fn new_rng(
        x: i32,
        y: i32,
        diameter: u32,
        vx: i32,
        vy: i32,
        multplier_max: f32,
        slow_start: f32,
    ) -> Self {
        let vx_rng = if rand::thread_rng().gen_bool(0.5) {
            (vx as f32 / slow_start) as i32
        } else {
            (-vx as f32 / slow_start) as i32
        };
        let vy_rng = ((rand::thread_rng().gen_range(-1000..=1000) * vy) as f32 / 1000.0) as i32;
        let rect = Rect::new(x, y, diameter, diameter);
        Self {
            rect,
            vx: vx_rng,
            vy: vy_rng,
            rect_default: rect,
            vx_default: vx.abs(),
            vy_default: vy.abs(),
            multplier_max,
            slow_start,
        }
    }

    pub fn after_goal_rng(&mut self, x: i32, y: i32, direction: TeamName) {
        self.rect = Rect::new(x, y, self.width() as u32, self.height() as u32);
        self.vx = match direction {
            TeamName::Right => (self.vx_default() as f32 / self.slow_start()) as i32,
            TeamName::Left => -(self.vx_default() as f32 / self.slow_start()) as i32,
        };
        self.vy = ((rand::thread_rng().gen_range(-1000..=1000) * self.vy_default()) as f32 / 1000.0)
            as i32;
    }

    pub fn x(&self) -> i32 {
        self.rect.x
    }

    pub fn y(&self) -> i32 {
        self.rect.y
    }

    pub fn vx(&self) -> i32 {
        self.vx
    }

    pub fn vy(&self) -> i32 {
        self.vy
    }

    pub fn vx_default(&self) -> i32 {
        self.vx_default
    }

    pub fn vy_default(&self) -> i32 {
        self.vy_default
    }

    pub fn multplier_max(&self) -> f32 {
        self.multplier_max
    }

    pub fn slow_start(&self) -> f32 {
        self.slow_start
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn width(&self) -> i32 {
        self.rect.width() as i32
    }

    pub fn height(&self) -> i32 {
        self.rect.height() as i32
    }

    pub fn rect_default(&self) -> Rect {
        self.rect_default
    }

    fn move_x_unchecked(&mut self, next_x: i32) {
        self.rect.x = next_x;
    }

    fn move_y_unchecked(&mut self, next_y: i32) {
        self.rect.y = next_y;
    }

    fn modify_vx_unchecked(&mut self, next_vx: i32) {
        self.vx = next_vx;
    }

    fn modify_vy_unchecked(&mut self, next_vy: i32) {
        self.vy = next_vy;
    }

    pub fn move_up(&mut self, steps: u32, top: i32) {
        let virtual_y = self.y() - steps as i32;
        let next_y = if virtual_y > top { virtual_y } else { top };
        self.move_y_unchecked(next_y);
    }

    pub fn move_down(&mut self, steps: u32, bottom: i32) {
        let virtual_y = self.y() + steps as i32;
        let next_y = if virtual_y + self.height() < bottom {
            virtual_y
        } else {
            bottom - self.height()
        };
        self.move_y_unchecked(next_y);
    }

    pub fn change_position(
        &mut self,
        paddles: Option<(&Paddle, &Paddle)>,
        x_min: i32,
        x_max: i32,
        y_min: i32,
        y_max: i32,
    ) -> Option<TeamName> {
        let walls = self.build_walls(paddles, x_min, x_max, y_min, y_max);
        let mut step_x = History::new(3, i32::MAX);
        step_x.push(self.vx());
        let mut step_y = History::new(3, i32::MAX);
        step_y.push(self.vy());

        loop {
            let vb_1 = if step_x.first() >= 0 {
                let id = 0;
                self.virtual_position_move_right(id, &walls[id], step_x.first(), step_y.first())
            } else {
                let id = 1;
                self.virtual_position_move_left(id, &walls[id], step_x.first(), step_y.first())
            };

            let vb_2 = if step_y.first() >= 0 {
                let id = 2;
                self.virtual_position_move_down(id, &walls[id], step_x.first(), step_y.first())
            } else {
                let id = 3;
                self.virtual_position_move_up(id, &walls[id], step_x.first(), step_y.first())
            };

            let step_left_1 = vb_1.step_x().abs() + vb_1.step_y().abs();
            let step_left_2 = vb_2.step_x().abs() + vb_2.step_y().abs();

            let longer_first = step_left_1 >= step_left_2;
            let vb = if longer_first { vb_1 } else { vb_2 };

            self.move_x_unchecked(vb.x());
            self.move_y_unchecked(vb.y());

            if vb.indexes().is_none() {
                return None;
            }
            let (id, index) = vb.indexes().expect("It can't be None");

            if id < 2 {
                let next_vx = self.vx_default().abs() * vb.step_x().signum();
                self.modify_vx_unchecked(next_vx);
                if index != 0 {
                    let min = walls[id][index].y_min() - self.height();
                    let max = walls[id][index].y_max();
                    let mid = (min + max) / 2;
                    let mul = self.multplier_max() * (vb.y() - mid) as f32 / (mid - min) as f32;
                    let next_vy = (self.vy_default() as f32 * mul) as i32;
                    self.modify_vy_unchecked(next_vy);
                }
            } else {
                let next_vy = self.vy().abs() * vb.step_y().signum();
                self.modify_vy_unchecked(next_vy);
            }

            step_x.push(vb.step_x());
            step_y.push(vb.step_y());

            if id == 0 && index == 0 {
                return Some(TeamName::Left);
            }
            if id == 1 && index == 0 {
                return Some(TeamName::Right);
            }

            let stuck = step_x.first() == step_x.get_value(2).expect("Index too high")
                && step_y.first() == step_y.get_value(2).expect("Index too high");
            if stuck {
                return None;
            }
        }
    }

    fn virtual_position_move_right(
        &self,
        id: usize,
        walls_right: &[Segmet2D],
        step_x: i32,
        step_y: i32,
    ) -> VirtualBall {
        let mut vb = VirtualBall::new(self.x() + step_x, self.y() + step_y, 0, 0, None);
        for i in 0..walls_right.len() {
            let movement_top_right = Segmet2D::new(
                self.x() + self.width(),
                self.y(),
                vb.x() + self.width(),
                vb.y(),
            );
            let movement_bottom_right = Segmet2D::new(
                self.x() + self.width(),
                self.y() + self.height(),
                vb.x() + self.width(),
                vb.y() + self.height(),
            );
            if let Some((x, y)) = movement_top_right.intersect(&walls_right[i]) {
                vb = VirtualBall::new(
                    x - self.width(),
                    y,
                    (x - self.width()) - vb.x(),
                    vb.y() - y,
                    if (x - self.width()) - vb.x() != 0 {
                        Some((id, i))
                    } else {
                        None
                    },
                );
            } else if let Some((x, y)) = movement_bottom_right.intersect(&walls_right[i]) {
                vb = VirtualBall::new(
                    x - self.width(),
                    y - self.height(),
                    x - self.width() - vb.x(),
                    vb.y() - (y - self.height()),
                    if x - self.width() - vb.x() != 0 {
                        Some((id, i))
                    } else {
                        None
                    },
                );
            }
        }
        vb
    }

    fn virtual_position_move_left(
        &self,
        id: usize,
        walls_left: &[Segmet2D],
        step_x: i32,
        step_y: i32,
    ) -> VirtualBall {
        let mut vb = VirtualBall::new(self.x() + step_x, self.y() + step_y, 0, 0, None);
        for i in 0..walls_left.len() {
            let movement_top_left = Segmet2D::new(self.x(), self.y(), vb.x(), vb.y());
            let movement_bottom_left = Segmet2D::new(
                self.x(),
                self.y() + self.height(),
                vb.x(),
                vb.y() + self.height(),
            );
            if let Some((x, y)) = movement_top_left.intersect(&walls_left[i]) {
                vb = VirtualBall::new(
                    x,
                    y,
                    x - vb.x(),
                    vb.y() - y,
                    if x - vb.x() != 0 { Some((id, i)) } else { None },
                );
            } else if let Some((x, y)) = movement_bottom_left.intersect(&walls_left[i]) {
                vb = VirtualBall::new(
                    x,
                    y - self.height(),
                    x - vb.x(),
                    vb.y() - (y - self.height()),
                    if x - vb.x() != 0 { Some((id, i)) } else { None },
                );
            }
        }
        vb
    }

    fn virtual_position_move_up(
        &self,
        id: usize,
        walls_up: &[Segmet2D],
        step_x: i32,
        step_y: i32,
    ) -> VirtualBall {
        let mut vb = VirtualBall::new(self.x() + step_x, self.y() + step_y, 0, 0, None);
        for i in 0..walls_up.len() {
            let movement_top_left = Segmet2D::new(self.x(), self.y(), vb.x(), vb.y());
            let movement_top_right = Segmet2D::new(
                self.x() + self.width(),
                self.y(),
                vb.x() + self.width(),
                vb.y(),
            );
            if let Some((x, y)) = movement_top_left.intersect(&walls_up[i]) {
                vb = VirtualBall::new(
                    x,
                    y,
                    vb.x() - x,
                    y - vb.y(),
                    if y - vb.y() != 0 { Some((id, i)) } else { None },
                );
            } else if let Some((x, y)) = movement_top_right.intersect(&walls_up[i]) {
                vb = VirtualBall::new(
                    x - self.width(),
                    y,
                    vb.x() - (x - self.width()),
                    y - vb.y(),
                    if y - vb.y() != 0 { Some((id, i)) } else { None },
                );
            }
        }
        vb
    }

    fn virtual_position_move_down(
        &self,
        id: usize,
        walls_down: &[Segmet2D],
        step_x: i32,
        step_y: i32,
    ) -> VirtualBall {
        let mut vb = VirtualBall::new(self.x() + step_x, self.y() + step_y, 0, 0, None);
        for i in 0..walls_down.len() {
            let movement_bottom_left = Segmet2D::new(
                self.x(),
                self.y() + self.height(),
                vb.x(),
                vb.y() + self.height(),
            );
            let movement_bottom_right = Segmet2D::new(
                self.x() + self.width(),
                self.y() + self.height(),
                vb.x() + self.width(),
                vb.y() + self.height(),
            );
            if let Some((x, y)) = movement_bottom_left.intersect(&walls_down[i]) {
                vb = VirtualBall::new(
                    x,
                    y - self.height(),
                    vb.x() - x,
                    (y - self.height()) - vb.y(),
                    if (y - self.height()) - vb.y() != 0 {
                        Some((id, i))
                    } else {
                        None
                    },
                );
            } else if let Some((x, y)) = movement_bottom_right.intersect(&walls_down[i]) {
                vb = VirtualBall::new(
                    x - self.width(),
                    y - self.height(),
                    vb.x() - (x - self.width()),
                    (y - self.height()) - vb.y(),
                    if (y - self.height()) - vb.y() != 0 {
                        Some((id, i))
                    } else {
                        None
                    },
                );
            }
        }
        vb
    }

    fn build_walls(
        &self,
        paddles: Option<(&Paddle, &Paddle)>,
        x_min: i32,
        x_max: i32,
        y_min: i32,
        y_max: i32,
    ) -> Vec<Vec<Segmet2D>> {
        let mut walls = vec![vec![]; 4];
        walls[0].push(Segmet2D::new(
            x_max,
            y_min - self.vy().abs(),
            x_max,
            y_max + self.vy().abs(),
        ));
        walls[1].push(Segmet2D::new(
            x_min,
            y_min - self.vy().abs(),
            x_min,
            y_max + self.vy().abs(),
        ));
        walls[2].push(Segmet2D::new(
            x_min - self.vx().abs(),
            y_max,
            x_max + self.vx().abs(),
            y_max,
        ));
        walls[3].push(Segmet2D::new(
            x_min - self.vx().abs(),
            y_min,
            x_max + self.vx().abs(),
            y_min,
        ));
        if let Some((paddle_l, paddle_r)) = paddles {
            walls[0].push(Segmet2D::new(
                paddle_l.x(),
                paddle_l.y(),
                paddle_l.x(),
                paddle_l.y() + paddle_l.height(),
            ));
            walls[0].push(Segmet2D::new(
                paddle_r.x(),
                paddle_r.y(),
                paddle_r.x(),
                paddle_r.y() + paddle_r.height(),
            ));
            walls[1].push(Segmet2D::new(
                paddle_l.x() + paddle_l.width(),
                paddle_l.y(),
                paddle_l.x() + paddle_l.width(),
                paddle_l.y() + paddle_l.height(),
            ));
            walls[1].push(Segmet2D::new(
                paddle_r.x() + paddle_r.width(),
                paddle_r.y(),
                paddle_r.x() + paddle_r.width(),
                paddle_r.y() + paddle_r.height(),
            ));
            walls[2].push(Segmet2D::new(
                paddle_l.x(),
                paddle_l.y(),
                paddle_l.x() + paddle_l.width(),
                paddle_l.y(),
            ));
            walls[2].push(Segmet2D::new(
                paddle_r.x(),
                paddle_r.y(),
                paddle_r.x() + paddle_r.width(),
                paddle_r.y(),
            ));
            walls[3].push(Segmet2D::new(
                paddle_l.x(),
                paddle_l.y() + paddle_l.height(),
                paddle_l.x() + paddle_l.width(),
                paddle_l.y() + paddle_l.height(),
            ));
            walls[3].push(Segmet2D::new(
                paddle_r.x(),
                paddle_r.y() + paddle_r.height(),
                paddle_r.x() + paddle_r.width(),
                paddle_r.y() + paddle_r.height(),
            ));
        }
        walls
    }
}
