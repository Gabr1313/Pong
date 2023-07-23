use crate::Result;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub struct DashedLineVert<'a> {
    texture: Texture<'a>,
    pos: Rect,
}

impl<'a> DashedLineVert<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        canvas: &mut Canvas<Window>,
        pos: Rect,
        number_of_segments: u32,
        segment_height_proportion: u32,
        space_height_proportion: u32,
        color: Color,
        background_color: Color,
    ) -> Result<Self> {
        let mut texture = texture_creator.create_texture_target(
            None,
            1,
            (segment_height_proportion + space_height_proportion) * number_of_segments,
        )?;
        canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_draw_color(background_color);
            texture_canvas.clear();
            texture_canvas.set_draw_color(color);
            let mut pos = 0;
            for _ in 0..number_of_segments {
                texture_canvas
                    .fill_rect(Rect::new(0, pos, 1, segment_height_proportion))
                    .unwrap();
                pos += (segment_height_proportion + space_height_proportion) as i32;
            }
            texture_canvas.set_draw_color(background_color);
        })?;
        Ok(Self { texture, pos })
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<()> {
        canvas.copy(&self.texture, None, self.pos)?;
        Ok(())
    }
}
