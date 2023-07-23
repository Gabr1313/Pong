use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::{Window, WindowContext};

use crate::constants::*;
use crate::team::TeamName;
use crate::Result;
use std::collections::HashMap;
use std::rc::Rc;

const X_PIXEL: u32 = 5;
const Y_PIXEL: u32 = 7;

pub struct PointDisplay<'a> {
    point_left: u32,
    point_right: u32,
    textures_left: Vec<Rc<Texture<'a>>>,
    textures_right: Vec<Rc<Texture<'a>>>,
    rects_left: Vec<Rect>,
    rects_right: Vec<Rect>,
    textures_hm: HashMap<char, Rc<Texture<'a>>>,
}

impl<'a> PointDisplay<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        canvas: &mut Canvas<Window>,
    ) -> Result<Self> {
        let mut point_display = Self {
            point_left: 0,
            point_right: 0,
            textures_left: vec![],
            textures_right: vec![],
            rects_left: vec![],
            rects_right: vec![],
            textures_hm: create_all_texture(canvas, texture_creator)?,
        };
        point_display.reset()?;
        Ok(point_display)
    }

    pub fn reset(&mut self) -> Result<()> {
        let points = vec!['0'];
        self.point_left = 0;
        self.update_textures(TeamName::Left, &points)?;
        self.update_rects(TeamName::Left, &points)?;
        self.point_right = 0;
        self.update_textures(TeamName::Right, &points)?;
        self.update_rects(TeamName::Right, &points)?;
        Ok(())
    }

    pub fn left(&mut self) -> u32 {
        self.point_left
    }

    pub fn right(&mut self) -> u32 {
        self.point_right
    }

    pub fn incr_point(&mut self, team: TeamName) -> Result<()> {
        let digits: Vec<_> = match team {
            TeamName::Left => {
                self.point_left += 1;
                self.point_left.to_string().chars().collect()
            }
            TeamName::Right => {
                self.point_right += 1;
                self.point_right.to_string().chars().collect()
            }
        };
        self.update_textures(team, &digits)?;
        self.update_rects(team, &digits)
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<()> {
        for (texture, rect) in self.textures_left.iter().zip(self.rects_left.iter()) {
            canvas.copy(texture, None, *rect)?;
        }
        for (texture, rect) in self.textures_right.iter().zip(self.rects_right.iter()) {
            canvas.copy(texture, None, *rect)?;
        }
        Ok(())
    }

    fn update_textures(&mut self, team: TeamName, digits: &Vec<char>) -> Result<()> {
        match team {
            TeamName::Left => {
                overwrite_textures(&mut self.textures_left, &self.textures_hm, digits)
            }
            TeamName::Right => {
                overwrite_textures(&mut self.textures_right, &self.textures_hm, digits)
            }
        }
    }

    fn update_rects(&mut self, team: TeamName, digits: &Vec<char>) -> Result<()> {
        match team {
            TeamName::Left => overwrite_rects_left(&mut self.rects_left, digits),
            TeamName::Right => overwrite_rects_right(&mut self.rects_right, digits),
        }
    }
}

fn overwrite_rects_right(rects: &mut Vec<Rect>, digits: &Vec<char>) -> Result<()> {
    let len_before = rects.len();
    rects.resize(digits.len(), Rect::new(0, 0, 0, 0));

    for i in len_before..rects.len() {
        rects[i] = Rect::new(
            ((WINDOW_WIDTH + MID_LINE_WIDTH) / 2
                + DISPLAY_COEFFICENT * (1 + i as u32 * (1 + X_PIXEL))) as i32,
            DISPLAY_COEFFICENT as i32,
            DISPLAY_COEFFICENT * X_PIXEL,
            DISPLAY_COEFFICENT * Y_PIXEL,
        );
    }
    Ok(())
}

fn overwrite_rects_left(rects: &mut Vec<Rect>, digits: &Vec<char>) -> Result<()> {
    let len_before = rects.len();
    rects.resize(digits.len(), Rect::new(0, 0, 0, 0));

    for i in len_before..rects.len() {
        rects.rotate_right(1);
        rects[0] = Rect::new(
            ((WINDOW_WIDTH - MID_LINE_WIDTH) / 2
                - DISPLAY_COEFFICENT * (i + 1) as u32 * (X_PIXEL + 1)) as i32,
            DISPLAY_COEFFICENT as i32,
            DISPLAY_COEFFICENT * X_PIXEL,
            DISPLAY_COEFFICENT * Y_PIXEL,
        );
    }
    Ok(())
}

fn overwrite_textures<'a>(
    textures: &mut Vec<Rc<Texture<'a>>>,
    textures_hm: &HashMap<char, Rc<Texture<'a>>>,
    digits: &Vec<char>,
) -> Result<()> {
    textures.resize(
        digits.len(),
        Rc::clone(
            textures_hm
                .get(&'0')
                .ok_or("character not found in the hashmap")?,
        ),
    );
    for (texture, digit) in textures.iter_mut().zip(digits.iter()) {
        *texture = Rc::clone(
            textures_hm
                .get(digit)
                .ok_or("character not found in the hashmap")?,
        );
    }
    Ok(())
}

fn create_all_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<HashMap<char, Rc<Texture<'a>>>> {
    let display_char: [(char, u64); 36] = [
        ('0', 0b01110100011001110101110011000101110),
        ('1', 0b01110001000010000100001000011000100),
        ('2', 0b11111000010001001100100001000101110),
        ('3', 0b01110100011000001100100001000101110),
        ('4', 0b01000010001111101001010100110001000),
        ('5', 0b01110100011000010000011110000111111),
        ('6', 0b01110100011000101111000011000101110),
        ('7', 0b00001000010001000100010001000011111),
        ('8', 0b01110100011000101110100011000101110),
        ('9', 0b01110100011000011110100011000101110),
        ('a', 0b10001100011000111111100011000111110),
        ('b', 0b01111100011000101111100011000101111),
        ('c', 0b01110100010000100001000011000101110),
        ('d', 0b01111100011000110001100011000101111),
        ('e', 0b11111000010000101111000010000111111),
        ('f', 0b00001000010000101111000010000111111),
        ('g', 0b01110100011100100001000011000101110),
        ('h', 0b10001100011000111111100011000110001),
        ('i', 0b01110001000010000100001000010001110),
        ('j', 0b01110100011000110000100001000011000),
        ('k', 0b10001010010010100011001010100110001),
        ('l', 0b11111000010000100001000010000100001),
        ('m', 0b10001100011000110001101011101110001),
        ('n', 0b10001100011100110101100111000110001),
        ('o', 0b01110100011000110001100011000101110),
        ('p', 0b00001000010000101111100011000101111),
        ('q', 0b10110010011010110001100011000101110),
        ('r', 0b10001100011000101111100011000101111),
        ('s', 0b01110100011000001110000011000101110),
        ('t', 0b00100001000010000100001000010011111),
        ('u', 0b01110100011000110001100011000110001),
        ('v', 0b00100010101000110001100011000110001),
        ('w', 0b01010101011010110001100011000110001),
        ('x', 0b10001100010101000100010101000110001),
        ('y', 0b00100001000010000100010101000110001),
        ('z', 0b11111000010001000100010001000011111),
    ];

    let mut hm = HashMap::new();
    for (c, mut x) in display_char {
        let mut texture = texture_creator.create_texture_target(None, X_PIXEL, Y_PIXEL)?;
        canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_draw_color(BACKGROUND_COLOR);
            texture_canvas.clear();
            texture_canvas.set_draw_color(DISPLAY_COLOR);
            for i in 0..Y_PIXEL as i32 {
                for j in 0..X_PIXEL as i32 {
                    if x & 1 == 1 {
                        texture_canvas.fill_rect(Rect::new(j, i, 1, 1)).unwrap();
                    }
                    x >>= 1;
                }
            }
        })?;
        hm.insert(c, Rc::new(texture));
    }
    Ok(hm)
}
