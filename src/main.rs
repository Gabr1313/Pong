use pong::ball::Ball;
use pong::constants::*;
use pong::game_status::GameStatus;
use pong::mid_line::DashedLineVert;
use pong::paddle::Paddle;
use pong::point_display::PointDisplay;
use pong::{play_game, suspended_game};
use sdl2::rect::Rect;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Pong", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas().accelerated().build()?;

    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();
    canvas.present();
    let mut events = sdl_context.event_pump()?;

    let texture_creator = canvas.texture_creator();
    let pos = Rect::new(
        (WINDOW_WIDTH - MID_LINE_WIDTH) as i32 / 2,
        0,
        MID_LINE_WIDTH,
        WINDOW_HEIGHT,
    );
    let mid_line = DashedLineVert::new(
        &texture_creator,
        &mut canvas,
        pos,
        MID_LINE_SEGMENTS,
        1,
        1,
        MID_LINE_COLOR,
        BACKGROUND_COLOR,
    )?;

    let mut point_display = PointDisplay::new(&texture_creator, &mut canvas)?;
    let mut paddle_l = Paddle::new(
        PADDLE_L_X,
        (WINDOW_HEIGHT - PADDLE_L_HEIGHT) as i32 / 2,
        PADDLE_WIDTH,
        PADDLE_L_HEIGHT,
        PADDLE_L_STEP,
    );
    let mut paddle_r = Paddle::new(
        PADDLE_R_X,
        (WINDOW_HEIGHT - PADDLE_R_HEIGHT) as i32 / 2,
        PADDLE_WIDTH,
        PADDLE_R_HEIGHT,
        PADDLE_R_STEP,
    );
    let mut ball = Ball::new_rng(
        (WINDOW_WIDTH - BALL_DIAMETER) as i32 / 2,
        (WINDOW_HEIGHT - BALL_DIAMETER) as i32 / 2,
        BALL_DIAMETER,
        BALL_VX,
        BALL_VY,
        MULTIPLIER,
        SLOW_START,
    );

    match suspended_game(
        &mut canvas,
        &mut point_display,
        &mut events,
        &mid_line,
        &mut paddle_l,
        &mut paddle_r,
        &mut ball,
    )? {
        GameStatus::Quit => return Ok(()),
        _ => {}
    }

    loop {
        point_display = PointDisplay::new(&texture_creator, &mut canvas)?;
        paddle_l = Paddle::new(
            PADDLE_L_X,
            (WINDOW_HEIGHT - PADDLE_L_HEIGHT) as i32 / 2,
            PADDLE_WIDTH,
            PADDLE_L_HEIGHT,
            PADDLE_L_STEP,
        );
        paddle_r = Paddle::new(
            PADDLE_R_X,
            (WINDOW_HEIGHT - PADDLE_R_HEIGHT) as i32 / 2,
            PADDLE_WIDTH,
            PADDLE_R_HEIGHT,
            PADDLE_R_STEP,
        );
        ball = Ball::new_rng(
            (WINDOW_WIDTH - BALL_DIAMETER) as i32 / 2,
            (WINDOW_HEIGHT - BALL_DIAMETER) as i32 / 2,
            BALL_DIAMETER,
            BALL_VX,
            BALL_VY,
            MULTIPLIER,
            SLOW_START,
        );

        match play_game(
            &mut canvas,
            &mut point_display,
            &mut events,
            &mid_line,
            &mut paddle_l,
            &mut paddle_r,
            &mut ball,
        )? {
            GameStatus::Quit => break,
            GameStatus::Reset => continue,
            _ => {}
        }

        match suspended_game(
            &mut canvas,
            &mut point_display,
            &mut events,
            &mid_line,
            &mut paddle_l,
            &mut paddle_r,
            &mut ball,
        )? {
            GameStatus::Quit => break,
            _ => {}
        }
    }
    Ok(())
}
