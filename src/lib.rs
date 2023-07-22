pub mod utils;
pub use crate::utils::*;
pub mod constants;
pub use crate::constants::*;

use std::error::Error;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::thread;
use std::time::{Duration, SystemTime};
use utils::game_status::GameStatus;

use ball::Ball;
use mid_line::DashedLineVert;
use paddle::Paddle;
use point_display::PointDisplay;

pub fn play_game<'a>(
    canvas: &mut Canvas<Window>,
    point_display: &'a mut PointDisplay,
    events: &mut EventPump,
    mid_line: &DashedLineVert,
    paddle_l: &mut Paddle,
    paddle_r: &mut Paddle,
    ball: &mut Ball,
) -> Result<GameStatus, Box<dyn Error>> {
    let mut loop_start_time = SystemTime::now();
    let mut elapsed_time;
    loop {
        elapsed_time = SystemTime::now().duration_since(loop_start_time)?;
        let frame_duration = Duration::from_nanos(1_000_000_000u64 / FPS);
        if frame_duration > elapsed_time {
            thread::sleep(Duration::from_nanos(1_000_000_000u64 / FPS) - elapsed_time);
        }
        loop_start_time = SystemTime::now();

        if let Some(mut status) = check_events(events) {
            if status == GameStatus::Pause {
                status = paused_game(
                    canvas,
                    point_display,
                    events,
                    mid_line,
                    paddle_l,
                    paddle_r,
                    ball,
                )?;
            }
            if status != GameStatus::Pause {
                return Ok(status);
            }
        }

        for key_pressed in events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
        {
            match key_pressed {
                PADDLE_L_UP => paddle_l.move_up(0, ball),
                PADDLE_L_DOWN => paddle_l.move_down(WINDOW_HEIGHT as i32, ball),
                PADDLE_R_UP => paddle_r.move_up(0, ball),
                PADDLE_R_DOWN => paddle_r.move_down(WINDOW_HEIGHT as i32, ball),
                _ => {}
            }
        }

        let points = ball.change_position(
            Some((&paddle_l, &paddle_r)),
            0,
            WINDOW_WIDTH as i32,
            0,
            WINDOW_HEIGHT as i32,
        );

        if let Some(team) = points {
            point_display.incr_point(canvas, team)?;
            ball.after_goal_rng(
                (WINDOW_WIDTH - BALL_DIAMETER) as i32 / 2,
                (WINDOW_HEIGHT - BALL_DIAMETER) as i32 / 2,
                team,
            );
        }

        if point_display.left() == POINT_TO_WIN || point_display.right() == POINT_TO_WIN {
            return Ok(GameStatus::End);
        }
        draw(
            canvas,
            point_display,
            mid_line,
            paddle_l,
            paddle_r,
            PADDLE_COLOR,
            ball,
            BALL_COLOR,
        )?;
    }
}

pub fn suspended_game<'a>(
    canvas: &mut Canvas<Window>,
    point_display: &'a mut PointDisplay,
    events: &mut EventPump,
    mid_line: &DashedLineVert,
    paddle_l: &mut Paddle,
    paddle_r: &mut Paddle,
    ball: &mut Ball,
) -> Result<GameStatus, Box<dyn Error>> {
    let mut loop_start_time = SystemTime::now();
    let mut elapsed_time;
    loop {
        elapsed_time = SystemTime::now().duration_since(loop_start_time)?;
        let frame_duration = Duration::from_nanos(1_000_000_000u64 / FPS);
        if frame_duration > elapsed_time {
            thread::sleep(Duration::from_nanos(1_000_000_000u64 / FPS) - elapsed_time);
        } else {
            eprintln!("Too slow");
        }
        loop_start_time = SystemTime::now();

        if let Some(status) = check_events(events) {
            if status != GameStatus::Pause {
                return Ok(status);
            }
        }
        ball.change_position(None, 0, WINDOW_WIDTH as i32, 0, WINDOW_HEIGHT as i32);
        draw(
            canvas,
            point_display,
            mid_line,
            paddle_l,
            paddle_r,
            PADDLE_COLOR_END,
            ball,
            BALL_COLOR,
        )?;
    }
}

pub fn paused_game<'a>(
    canvas: &mut Canvas<Window>,
    point_display: &'a mut PointDisplay,
    events: &mut EventPump,
    mid_line: &DashedLineVert,
    paddle_l: &mut Paddle,
    paddle_r: &mut Paddle,
    ball: &mut Ball,
) -> Result<GameStatus, Box<dyn Error>> {
    let mut loop_start_time = SystemTime::now();
    let mut elapsed_time;
    draw(
        canvas,
        point_display,
        mid_line,
        paddle_l,
        paddle_r,
        PADDLE_COLOR_PAUSE,
        ball,
        BALL_COLOR,
    )?;
    loop {
        elapsed_time = SystemTime::now().duration_since(loop_start_time)?;
        let frame_duration = Duration::from_nanos(1_000_000_000u64 / FPS);
        if frame_duration > elapsed_time {
            thread::sleep(Duration::from_nanos(1_000_000_000u64 / FPS) - elapsed_time);
        }
        loop_start_time = SystemTime::now();

        if let Some(status) = check_events(events) {
            return Ok(status);
        }
    }
}

fn draw<'a>(
    canvas: &mut Canvas<Window>,
    point_display: &'a PointDisplay,
    mid_line: &DashedLineVert,
    paddle_l: &Paddle,
    paddle_r: &Paddle,
    paddle_color: Color,
    ball: &Ball,
    ball_color: Color,
) -> Result<(), String> {
    canvas.clear();
    point_display.draw(canvas)?;
    mid_line.draw(canvas)?;
    canvas.set_draw_color(paddle_color);
    canvas.fill_rect(paddle_l.rect())?;
    canvas.fill_rect(paddle_r.rect())?;
    canvas.set_draw_color(ball_color);
    canvas.fill_rect(ball.rect())?;
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.present();
    Ok(())
}

fn check_events<'a>(events: &mut EventPump) -> Option<GameStatus> {
    for event in events.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(QUIT),
                ..
            } => return Some(GameStatus::Quit),
            Event::KeyDown {
                keycode: Some(RESET),
                ..
            } => return Some(GameStatus::Reset),
            Event::KeyDown {
                keycode: Some(PAUSE),
                ..
            } => return Some(GameStatus::Pause),
            _ => {}
        }
    }
    None
}
