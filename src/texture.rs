use crate::{math::vector::Vec2f32, utils};

#[derive(Debug, Clone, Copy, Default)]
pub struct TextureColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Texture {
    width: u32,
    height: u32,
    data: Box<[TextureColor]>,
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Self {
        Texture {
            width,
            height,
            data: vec![Default::default(); (width * height) as usize].into_boxed_slice(),
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set_color(&mut self, x: u32, y: u32, color: TextureColor) -> utils::Result<()> {
        if x >= self.width || y >= self.height {
            return Err("illegal arguments".into());
        }

        self.data[(x + y * self.width) as usize] = color;
        Ok(())
    }

    pub fn get_color(&self, x: u32, y: u32) -> utils::Result<TextureColor> {
        if x >= self.width || y >= self.height {
            return Err("illegal arguments".into());
        }

        Ok(self.data[(x + y * self.width) as usize])
    }

    pub fn flip_horizontally(&mut self) {
        let half_width = self.width >> 1;

        for i in 0..half_width {
            for j in 0..self.height {
                let pos_1 = (i + j * self.width) as usize;
                let pos_2 = ((self.width - i - 1) + j * self.width) as usize;

                self.data.swap(pos_1, pos_2);
            }
        }
    }

    pub fn flip_vertically(&mut self) {
        let half_height = self.height >> 1;

        for i in 0..self.width {
            for j in 0..half_height {
                let pos_1 = (i + j * self.width) as usize;
                let pos_2 = (i + (self.height - j - 1) * self.width) as usize;

                self.data.swap(pos_1, pos_2);
            }
        }
    }

    pub fn sample(&self, uv: &Vec2f32) -> TextureColor {
        self.get_color(
            (uv[0] * (self.width as f32)) as u32,
            (uv[1] * (self.height as f32)) as u32,
        )
        .unwrap()
    }
}

pub mod tga;
