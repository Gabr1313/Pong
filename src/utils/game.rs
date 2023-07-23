use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::thread;
use std::time::{Duration, SystemTime};

use crate::ball::Ball;
use crate::constants::*;
use crate::error::GameInfiniteLoop;
use crate::game_status::GameStatus;
use crate::mid_line::DashedLineVert;
use crate::paddle::Paddle;
use crate::point_display::PointDisplay;
use crate::Result;

pub struct Game<'a> {
    canvas: Canvas<Window>,
    point_display: PointDisplay<'a>,
    events: EventPump,
    mid_line: DashedLineVert<'a>,
    paddle_l: Paddle,
    paddle_r: Paddle,
    ball: Ball,
    fps: u64,
    status: GameStatus,
    prev_status: GameStatus,
}

impl<'a> Game<'a> {
    pub fn new(
        canvas: Canvas<Window>,
        point_display: PointDisplay<'a>,
        events: EventPump,
        mid_line: DashedLineVert<'a>,
        fps: u64,
    ) -> Self {
        Self {
            canvas,
            point_display,
            events,
            mid_line,
            paddle_l: Paddle::new(
                PADDLE_L_X,
                (WINDOW_HEIGHT - PADDLE_L_HEIGHT) as i32 / 2,
                PADDLE_WIDTH,
                PADDLE_L_HEIGHT,
                PADDLE_L_STEP,
            ),
            paddle_r: Paddle::new(
                PADDLE_R_X,
                (WINDOW_HEIGHT - PADDLE_R_HEIGHT) as i32 / 2,
                PADDLE_WIDTH,
                PADDLE_R_HEIGHT,
                PADDLE_R_STEP,
            ),
            ball: Ball::new_rng(
                (WINDOW_WIDTH - BALL_DIAMETER) as i32 / 2,
                (WINDOW_HEIGHT - BALL_DIAMETER) as i32 / 2,
                BALL_DIAMETER,
                BALL_VX,
                BALL_VY,
                MULTIPLIER,
                SLOW_START,
            ),
            fps,
            status: GameStatus::Waiting,
            prev_status: GameStatus::Play,
        }
    }
    pub fn spawn(&mut self) -> Result<()> {
        self.draw()?;
        let mut loop_start_time = SystemTime::now();
        let mut elapsed_time;
        let mut draw = true;
        loop {
            elapsed_time = SystemTime::now().duration_since(loop_start_time)?;
            let frame_duration = Duration::from_nanos(1_000_000_000u64 / self.fps);
            if frame_duration > elapsed_time {
                thread::sleep(Duration::from_nanos(1_000_000_000u64 / self.fps) - elapsed_time);
            }
            loop_start_time = SystemTime::now();
            self.update_status()?;
            if draw {
                match self.status {
                    GameStatus::Play => self.play()?,
                    GameStatus::Reset => self.reset()?,
                    GameStatus::End => self.end()?,
                    GameStatus::Waiting => self.waiting()?,
                    GameStatus::Quit => break,
                }
            }
            match self.status {
                GameStatus::Waiting => draw = false,
                GameStatus::Quit => break,
                _ => draw = true,
            }
        }
        Ok(())
    }

    fn play(&mut self) -> Result<()> {
        for key_pressed in self
            .events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
        {
            match key_pressed {
                PADDLE_L_UP => self.paddle_l.move_up(0, &mut self.ball),
                PADDLE_L_DOWN => self
                    .paddle_l
                    .move_down(WINDOW_HEIGHT as i32, &mut self.ball),
                PADDLE_R_UP => self.paddle_r.move_up(0, &mut self.ball),
                PADDLE_R_DOWN => self
                    .paddle_r
                    .move_down(WINDOW_HEIGHT as i32, &mut self.ball),
                _ => {}
            }
        }

        let points = self.ball.change_position(
            Some((&self.paddle_l, &self.paddle_r)),
            0,
            WINDOW_WIDTH as i32,
            0,
            WINDOW_HEIGHT as i32,
        );

        if let Some(team) = points? {
            self.point_display.incr_point(team)?;
            self.ball.after_goal_rng(
                (WINDOW_WIDTH - BALL_DIAMETER) as i32 / 2,
                (WINDOW_HEIGHT - BALL_DIAMETER) as i32 / 2,
                team,
            );
        }

        if self.point_display.left() == POINT_TO_WIN || self.point_display.right() == POINT_TO_WIN {
            self.change_status(GameStatus::End);
        }
        self.draw()?;
        Ok(())
    }

    fn change_status(&mut self, status: GameStatus) {
        self.prev_status = self.status;
        self.status = status;
    }

    fn draw(&mut self) -> Result<()> {
        self.draw_color(PADDLE_COLOR, BALL_COLOR, BACKGROUND_COLOR)?;
        Ok(())
    }

    fn draw_pause(&mut self) -> Result<()> {
        self.draw_color(PADDLE_COLOR_PAUSE, BALL_COLOR_PAUSE, BACKGROUND_COLOR)?;
        Ok(())
    }

    fn draw_color(
        &mut self,
        paddle_color: Color,
        ball_color: Color,
        background_color: Color,
    ) -> Result<()> {
        self.canvas.clear();
        self.point_display.draw(&mut self.canvas)?;
        self.mid_line.draw(&mut self.canvas)?;
        self.canvas.set_draw_color(paddle_color);
        self.canvas.fill_rect(self.paddle_l.rect())?;
        self.canvas.fill_rect(self.paddle_r.rect())?;
        self.canvas.set_draw_color(ball_color);
        self.canvas.fill_rect(self.ball.rect())?;
        self.canvas.set_draw_color(background_color);
        self.canvas.present();
        Ok(())
    }

    fn end(&mut self) -> Result<()> {
        self.ball
            .change_position(None, 0, WINDOW_WIDTH as i32, 0, WINDOW_HEIGHT as i32)?;
        self.draw_pause()?;
        Ok(())
    }

    fn waiting(&mut self) -> Result<()> {
        self.draw_pause()?;
        Ok(())
    }

    fn reset(&mut self) -> Result<()> {
        self.point_display.reset()?;
        self.paddle_l = Paddle::new(
            PADDLE_L_X,
            (WINDOW_HEIGHT - PADDLE_L_HEIGHT) as i32 / 2,
            PADDLE_WIDTH,
            PADDLE_L_HEIGHT,
            PADDLE_L_STEP,
        );
        self.paddle_r = Paddle::new(
            PADDLE_R_X,
            (WINDOW_HEIGHT - PADDLE_R_HEIGHT) as i32 / 2,
            PADDLE_WIDTH,
            PADDLE_R_HEIGHT,
            PADDLE_R_STEP,
        );
        self.ball = Ball::new_rng(
            (WINDOW_WIDTH - BALL_DIAMETER) as i32 / 2,
            (WINDOW_HEIGHT - BALL_DIAMETER) as i32 / 2,
            BALL_DIAMETER,
            BALL_VX,
            BALL_VY,
            MULTIPLIER,
            SLOW_START,
        );
        self.change_status(GameStatus::Play);
        Ok(())
    }

    fn update_status(&mut self) -> Result<()> {
        if let Some(status) = self.check_events()? {
            self.change_status(status);
        }
        Ok(())
    }

    fn check_events(&mut self) -> Result<Option<GameStatus>> {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(QUIT),
                    ..
                } => return Ok(Some(GameStatus::Quit)),
                Event::KeyDown {
                    keycode: Some(RESET),
                    ..
                } => return Ok(Some(GameStatus::Reset)),
                Event::KeyDown {
                    keycode: Some(PAUSE),
                    ..
                } => {
                    if self.status == GameStatus::Waiting {
                        if self.prev_status == GameStatus::Waiting {
                            return Err(Box::new(GameInfiniteLoop));
                        }
                        return Ok(Some(self.prev_status));
                    }
                    return Ok(Some(GameStatus::Waiting));
                }
                _ => {}
            }
        }
        Ok(None)
    }
}
