use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::{Window, WindowContext};

use crate::constants::*;
use crate::team::TeamName;
use std::error::Error;

pub struct PointDisplay<'a> {
    point_left: u32,
    point_right: u32,
    texture_left: Texture<'a>,
    texture_right: Texture<'a>,
    rect_left: Rect,
    rect_right: Rect,
}

impl<'a> PointDisplay<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        canvas: &mut Canvas<Window>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut s = Self {
            point_left: 0,
            point_right: 0,
            texture_left: texture_creator.create_texture_target(None, 7, 5)?,
            texture_right: texture_creator.create_texture_target(None, 7, 5)?,
            rect_left: Rect::new(
                (WINDOW_WIDTH / 2 - DISPLAY_WIDTH - DISPLAY_DISTANCE_CENTER) as i32,
                10,
                DISPLAY_WIDTH,
                DISPLAY_HEIGTH,
            ),
            rect_right: Rect::new(
                (WINDOW_WIDTH / 2 + DISPLAY_DISTANCE_CENTER) as i32,
                10,
                DISPLAY_WIDTH,
                DISPLAY_HEIGTH,
            ),
        };
        s.update_texture(canvas, TeamName::Left)?;
        s.update_texture(canvas, TeamName::Right)?;
        Ok(s)
    }

    pub fn reset(&mut self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>> {
        self.point_left = 0;
        self.point_right = 0;
        create_texture(&mut self.texture_left, self.point_left, canvas)?;
        create_texture(&mut self.texture_right, self.point_right, canvas)?;
        Ok(())
    }

    pub fn left(&mut self) -> u32 {
        self.point_left
    }

    pub fn right(&mut self) -> u32 {
        self.point_right
    }

    pub fn incr_point(
        &mut self,
        canvas: &mut Canvas<Window>,
        team: TeamName,
    ) -> Result<(), Box<dyn Error>> {
        match team {
            TeamName::Left => self.point_left += 1,
            TeamName::Right => self.point_right += 1,
        }
        self.update_texture(canvas, team)
    }
    fn update_texture(
        &mut self,
        canvas: &mut Canvas<Window>,
        team: TeamName,
    ) -> Result<(), Box<dyn Error>> {
        match team {
            TeamName::Left => create_texture(&mut self.texture_left, self.point_left, canvas),
            TeamName::Right => create_texture(&mut self.texture_right, self.point_right, canvas),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.copy(&self.texture_left, None, self.rect_left)?;

        canvas.copy(&self.texture_right, None, self.rect_right)?;
        Ok(())
    }
}

const DISPLAY_NUMBERS: [u16; 10] = [
    31599, 29850, 29671, 31207, 18925, 31183, 31695, 18727, 31727, 31215,
];

fn create_texture(
    texture: &mut Texture,
    n: u32,
    canvas: &mut Canvas<Window>,
) -> Result<(), Box<dyn Error>> {
    let first_digit = n as usize / 10;
    canvas.with_texture_canvas(texture, |texture_canvas| {
        texture_canvas.set_draw_color(BACKGROUND_COLOR);
        texture_canvas.clear();
        texture_canvas.set_draw_color(DISPLAY_COLOR);
        let mut x = DISPLAY_NUMBERS[first_digit];
        for i in 0..5 {
            for j in 0..3 {
                if x & 1 == 1 {
                    texture_canvas.fill_rect(Rect::new(j, i, 1, 1)).unwrap();
                }
                x >>= 1;
            }
        }
    })?;
    let second_digit = n as usize % 10;
    canvas.with_texture_canvas(texture, |texture_canvas| {
        let mut x = DISPLAY_NUMBERS[second_digit];
        for i in 0..5 {
            for j in 0..3 {
                if x & 1 == 1 {
                    texture_canvas.fill_rect(Rect::new(j + 4, i, 1, 1)).unwrap();
                }
                x >>= 1;
            }
        }
        texture_canvas.set_draw_color(BACKGROUND_COLOR);
    })?;
    Ok(())
}
