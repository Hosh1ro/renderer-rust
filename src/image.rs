type Result<T> = std::result::Result<T, &'static str>;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub struct Image {
    width: u32,
    height: u32,
    data: Box<[Color]>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            width,
            height,
            data: Vec::with_capacity((width * height) as usize).into_boxed_slice(),
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set_color(&mut self, x: u32, y: u32, c: Color) -> Result<()> {
        if x >= self.width || y >= self.height {
            return Err("Illegal arguments");
        }

        self.data[(x + y * self.width) as usize] = c;
        Ok(())
    }

    pub fn get_color(&self, x: u32, y: u32) -> Result<Color> {
        if x >= self.width || y >= self.height {
            return Err("Illegal arguments");
        }

        Ok(self.data[(x + y * self.width) as usize])
    }

    pub fn flip_horizontally(&mut self) {
        let half_width = self.width >> 1;

        for i in 0..half_width {
            for j in 0..self.get_height() {
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
}
