use pong::constants::*;
use pong::game::Game;
use pong::mid_line::DashedLineVert;
use pong::point_display::PointDisplay;
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
    let events = sdl_context.event_pump()?;

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

    let point_display = PointDisplay::new(&texture_creator, &mut canvas)?;

    let mut game = Game::new(canvas, point_display, events, mid_line, FPS);
    game.spawn()?;
    Ok(())
}
