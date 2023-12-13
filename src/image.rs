use crate::utils;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::Path,
};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Image {
    width: u32,
    height: u32,
    data: Box<[Color]>,
}

#[repr(C, packed(1))]
#[derive(Default)]
struct TgaHeader {
    image_id_length: u8,
    color_map_type: u8,
    image_type: u8,
    color_map_first_entry_index: u16,
    color_map_length: u16,
    color_map_entry_size: u8,
    x_origin: u16,
    y_origin: u16,
    image_width: u16,
    image_height: u16,
    pixel_depth: u8,
    image_descriptor: u8,
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

    pub fn set_color(&mut self, x: u32, y: u32, color: Color) -> utils::Result<()> {
        if x >= self.width || y >= self.height {
            return Err("illegal arguments".into());
        }

        self.data[(x + y * self.width) as usize] = color;
        Ok(())
    }

    pub fn get_color(&self, x: u32, y: u32) -> utils::Result<Color> {
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

    fn set_image_data(&mut self, raw_data: &Vec<u8>, pixel_depth: u32) {
        match pixel_depth {
            8 => {
                for i in 0..self.width {
                    for j in 0..self.height {
                        let index = (i + j * self.width) as usize;
                        let color = Color {
                            b: raw_data[index],
                            g: raw_data[index],
                            r: raw_data[index],
                            a: 255,
                        };

                        self.set_color(i, j, color).unwrap();
                    }
                }
            }
            24 => {
                for i in 0..self.width {
                    for j in 0..self.height {
                        let index = ((i + j * self.width) * 3) as usize;
                        let color = Color {
                            b: raw_data[index],
                            g: raw_data[index + 1],
                            r: raw_data[index + 2],
                            a: 255,
                        };

                        self.set_color(i, j, color).unwrap();
                    }
                }
            }
            32 => {
                for i in 0..self.width {
                    for j in 0..self.height {
                        let index = ((i + j * self.width) * 4) as usize;
                        let color = Color {
                            b: raw_data[index],
                            g: raw_data[index + 1],
                            r: raw_data[index + 2],
                            a: raw_data[index + 3],
                        };

                        self.set_color(i, j, color).unwrap();
                    }
                }
            }
            _ => {}
        }
    }

    fn get_image_data(&self, raw_data: &mut Vec<u8>) {
        for i in 0..self.width {
            for j in 0..self.height {
                let index = ((i + j * self.width) * 4) as usize;
                let color = self.get_color(i, j).unwrap();

                raw_data[index] = color.b;
                raw_data[index + 1] = color.g;
                raw_data[index + 2] = color.r;
                raw_data[index + 3] = color.a;
            }
        }
    }

    fn rle_decode(&mut self, reader: &mut BufReader<File>, pixel_depth: u32) -> utils::Result<()> {
        let total_pixel = (self.width * self.height) as usize;
        let mut current_pixel = 0usize;
        let get_color = |data: &Vec<u8>| match pixel_depth {
            8 => Color {
                b: data[0],
                g: data[0],
                r: data[0],
                a: 255,
            },
            24 => Color {
                b: data[0],
                g: data[1],
                r: data[2],
                a: 255,
            },
            32 => Color {
                b: data[0],
                g: data[1],
                r: data[2],
                a: data[3],
            },
            _ => Color {
                b: 0,
                g: 0,
                r: 0,
                a: 0,
            },
        };

        loop {
            let mut packet = utils::read_n_bytes(reader, 1)?[0];

            // repeated data packets if highest bit is 1
            if packet >= 128 {
                packet -= 127;
                let pixel_value = utils::read_n_bytes(reader, (pixel_depth >> 3) as u64)?;
                let color = get_color(&pixel_value);

                for _ in 0..packet {
                    if current_pixel >= total_pixel {
                        return Err("wrong pixel numbers".into());
                    }

                    self.data[current_pixel] = color;
                    current_pixel += 1;
                }
            } else {
                packet += 1;
                for _ in 0..packet {
                    if current_pixel >= total_pixel {
                        return Err("wrong pixel numbers".into());
                    }

                    let pixel_value = utils::read_n_bytes(reader, (pixel_depth >> 3) as u64)?;
                    let color = get_color(&pixel_value);
                    self.data[current_pixel] = color;
                    current_pixel += 1;
                }
            }

            if current_pixel >= total_pixel {
                break Ok(());
            }
        }
    }

    pub fn read_from_file(path: &Path) -> utils::Result<Self> {
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);

        let tga_header: TgaHeader = unsafe { utils::read_raw_struct(&mut reader)? };

        let (width, height, pixel_depth) = (
            tga_header.image_width as u32,
            tga_header.image_height as u32,
            tga_header.pixel_depth as u32,
        );
        if pixel_depth != 8 || pixel_depth != 24 || pixel_depth != 32 {
            return Err("unsupported file format".into());
        }

        let mut image = Image::new(width, height);
        let raw_data_size = (width * height * (pixel_depth >> 3)) as u64;

        match tga_header.image_type {
            2 => {
                let raw_data = utils::read_n_bytes(&mut reader, raw_data_size)?;
                image.set_image_data(&raw_data, pixel_depth);
            }
            3 => {
                let raw_data = utils::read_n_bytes(&mut reader, raw_data_size)?;
                image.set_image_data(&raw_data, pixel_depth);
            }
            10 => {
                image.rle_decode(&mut reader, pixel_depth)?;
            }
            11 => {
                image.rle_decode(&mut reader, pixel_depth)?;
            }
            _ => {
                return Err("unsupported file format".into());
            }
        }

        if (tga_header.image_descriptor >> 4) & 1 == 1 {
            image.flip_horizontally();
        }
        if (tga_header.image_descriptor >> 5) & 1 == 1 {
            image.flip_vertically();
        }

        return Ok(image);
    }

    pub fn write_to_file(&self, path: &Path) -> utils::Result<()> {
        let file = File::create(&path)?;
        let mut writer = BufWriter::new(file);

        let tga_header = TgaHeader {
            image_type: 2,
            image_width: self.width as u16,
            image_height: self.height as u16,
            pixel_depth: 32,
            image_descriptor: 0x20,
            ..Default::default()
        };

        unsafe { utils::write_raw_struct(&mut writer, &tga_header)? };

        let mut raw_data = Vec::<u8>::with_capacity((self.width * self.height * 4) as usize);
        self.get_image_data(&mut raw_data);

        writer.write_all(&raw_data)?;

        Ok(())
    }
}
