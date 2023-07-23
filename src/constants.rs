use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub const WINDOW_HEIGHT: u32 = 720;
pub const WINDOW_WIDTH: u32 = WINDOW_HEIGHT * 16 / 9;
pub const FPS: u64 = 60;
pub const POINT_TO_WIN: u32 = 15;

pub const RESET: Keycode = Keycode::R;
pub const QUIT: Keycode = Keycode::Escape;
pub const PAUSE: Keycode = Keycode::P;

pub const PADDLE_L_UP: Keycode = Keycode::W;
pub const PADDLE_L_DOWN: Keycode = Keycode::S;
pub const PADDLE_R_UP: Keycode = Keycode::Up;
pub const PADDLE_R_DOWN: Keycode = Keycode::Down;
pub const PADDLE_L_X: i32 = (WINDOW_WIDTH / 16 - PADDLE_WIDTH / 2) as i32;
pub const PADDLE_R_X: i32 = ((WINDOW_WIDTH - WINDOW_WIDTH / 16) - PADDLE_WIDTH / 2) as i32;
pub const PADDLE_L_HEIGHT: u32 = WINDOW_HEIGHT / 8;
pub const PADDLE_R_HEIGHT: u32 = WINDOW_HEIGHT / 8;
pub const PADDLE_WIDTH: u32 = WINDOW_WIDTH / 64;
pub const PADDLE_L_STEP: u32 = PADDLE_L_HEIGHT / 8;
pub const PADDLE_R_STEP: u32 = PADDLE_R_HEIGHT / 8;

pub const MID_LINE_WIDTH: u32 = WINDOW_WIDTH / 160;
pub const MID_LINE_SEGMENTS: u32 = 32;

pub const BALL_DIAMETER: u32 = (WINDOW_WIDTH + WINDOW_HEIGHT) / 140;
pub const BALL_VX: i32 = WINDOW_WIDTH as i32 / 100;
pub const BALL_VY: i32 = BALL_VX;
pub const MULTIPLIER: f32 = 2.0;
pub const SLOW_START: f32 = 1.5;

pub const DISPLAY_COEFFICENT: u32 = (WINDOW_WIDTH + WINDOW_HEIGHT) / 90;

pub const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);
pub const PADDLE_COLOR: Color = Color::RGB(255, 255, 255);
pub const PADDLE_COLOR_END: Color = Color::RGB(127, 127, 127);
pub const PADDLE_COLOR_PAUSE: Color = Color::RGB(127, 127, 127);
pub const MID_LINE_COLOR: Color = Color::RGB(255, 255, 255);
pub const BALL_COLOR: Color = Color::RGB(255, 255, 0);
pub const DISPLAY_COLOR: Color = Color::RGB(191, 191, 191);
