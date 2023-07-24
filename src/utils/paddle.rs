use crate::ball::Ball;
use crate::segment::Segmet1D;
use sdl2::rect::Rect;

#[derive(Debug)]
pub struct Paddle {
    rect: Rect,
    step: u32,
}

impl Paddle {
    pub fn new(x: i32, y: i32, width: u32, height: u32, step: u32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            step,
        }
    }
    pub fn x(&self) -> i32 {
        self.rect.x()
    }
    pub fn y(&self) -> i32 {
        self.rect.y()
    }
    pub fn height(&self) -> i32 {
        self.rect.height() as i32
    }
    pub fn width(&self) -> i32 {
        self.rect.width() as i32
    }
    pub fn rect(&self) -> Rect {
        self.rect
    }
    fn step(&self) -> i32 {
        self.step as i32
    }

    pub fn move_up(&mut self, top: i32, ball: &mut Ball) {
        let ball_bottom = ball.y() + ball.height();
        let ball_left = ball.x();
        let ball_right = ball.x() + ball.width();
        let paddle_top = self.y();
        let paddle_top_virtual = self.y() - self.step();
        let paddle_left = self.x();
        let paddle_right = self.x() + self.width();
        let s1 = Segmet1D::new(ball_left, ball_right);
        let s2 = Segmet1D::new(paddle_left, paddle_right);

        if (paddle_top >= ball_bottom && ball_bottom > paddle_top_virtual) && s1.intersect(&s2) {
            let virtual_ball_pos = paddle_top_virtual - ball.height();
            let step = (ball.y() - virtual_ball_pos).abs() as u32;
            ball.move_up(step, top);
            let ball_bottom = ball.y() + ball.height();
            self.rect.y = ball_bottom;
        } else if paddle_top_virtual >= top {
            self.rect.y = paddle_top_virtual;
        } else {
            self.rect.y = top;
        }
    }

    pub fn move_down(&mut self, bottom: i32, ball: &mut Ball) {
        let ball_top = ball.y();
        let ball_left = ball.x();
        let ball_right = ball.x() + ball.width();
        let paddle_bottom = self.y() + self.height();
        let paddle_bottom_virtual = self.y() + self.height() + self.step();
        let paddle_left = self.x();
        let paddle_right = self.x() + self.width();
        let s1 = Segmet1D::new(ball_left, ball_right);
        let s2 = Segmet1D::new(paddle_left, paddle_right);

        if (paddle_bottom <= ball_top && ball_top < paddle_bottom_virtual) && s1.intersect(&s2) {
            let virtual_ball_pos = paddle_bottom_virtual;
            let step = (ball.y() - virtual_ball_pos).abs() as u32;
            ball.move_down(step, bottom);
            let ball_top = ball.y();
            self.rect.y = ball_top - self.height();
        } else if self.y() + self.height() + self.step() <= bottom as i32 {
            self.rect.y = paddle_bottom_virtual - self.height();
        } else {
            self.rect.y = bottom - self.height();
        }
    }
}
