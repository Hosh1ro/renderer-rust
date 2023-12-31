use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::Path,
};

use crate::texture::{Texture, TextureColor};
use crate::utils;

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

fn set_image_data(texture: &mut Texture, raw_data: &Vec<u8>, pixel_depth: u32) {
    match pixel_depth {
        8 => {
            for i in 0..texture.width {
                for j in 0..texture.height {
                    let index = (i + j * texture.width) as usize;
                    let color = TextureColor {
                        b: raw_data[index],
                        g: raw_data[index],
                        r: raw_data[index],
                        a: 255,
                    };

                    texture.set_color(i, j, color).unwrap();
                }
            }
        }
        24 => {
            for i in 0..texture.width {
                for j in 0..texture.height {
                    let index = ((i + j * texture.width) * 3) as usize;
                    let color = TextureColor {
                        b: raw_data[index],
                        g: raw_data[index + 1],
                        r: raw_data[index + 2],
                        a: 255,
                    };

                    texture.set_color(i, j, color).unwrap();
                }
            }
        }
        32 => {
            for i in 0..texture.width {
                for j in 0..texture.height {
                    let index = ((i + j * texture.width) * 4) as usize;
                    let color = TextureColor {
                        b: raw_data[index],
                        g: raw_data[index + 1],
                        r: raw_data[index + 2],
                        a: raw_data[index + 3],
                    };

                    texture.set_color(i, j, color).unwrap();
                }
            }
        }
        _ => {}
    }
}

fn get_image_data(texture: &Texture, raw_data: &mut Vec<u8>, ignore_alpha: bool) {
    for i in 0..texture.width {
        for j in 0..texture.height {
            let index = ((i + j * texture.width) * 4) as usize;
            let color = texture.get_color(i, j).unwrap();

            raw_data[index] = color.b;
            raw_data[index + 1] = color.g;
            raw_data[index + 2] = color.r;
            raw_data[index + 3] = if ignore_alpha { 255 } else { color.a };
        }
    }
}

fn rle_decode(
    texture: &mut Texture,
    reader: &mut BufReader<File>,
    pixel_depth: u32,
) -> utils::Result<()> {
    let total_pixel = (texture.width * texture.height) as usize;
    let mut current_pixel = 0usize;
    let get_color = |data: &Vec<u8>| match pixel_depth {
        8 => TextureColor {
            b: data[0],
            g: data[0],
            r: data[0],
            a: 255,
        },
        24 => TextureColor {
            b: data[0],
            g: data[1],
            r: data[2],
            a: 255,
        },
        32 => TextureColor {
            b: data[0],
            g: data[1],
            r: data[2],
            a: data[3],
        },
        _ => TextureColor {
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

                texture.data[current_pixel] = color;
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
                texture.data[current_pixel] = color;
                current_pixel += 1;
            }
        }

        if current_pixel >= total_pixel {
            break Ok(());
        }
    }
}

pub fn read_from_file(path: &Path) -> utils::Result<Texture> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);

    let tga_header: TgaHeader = unsafe { utils::read_raw_struct(&mut reader)? };

    let (width, height, pixel_depth) = (
        tga_header.image_width as u32,
        tga_header.image_height as u32,
        tga_header.pixel_depth as u32,
    );
    if pixel_depth != 8 && pixel_depth != 24 && pixel_depth != 32 {
        return Err("unsupported file format".into());
    }

    let mut texture = Texture::new(width, height);
    let raw_data_size = (width * height * (pixel_depth >> 3)) as u64;

    match tga_header.image_type {
        2 => {
            let raw_data = utils::read_n_bytes(&mut reader, raw_data_size)?;
            set_image_data(&mut texture, &raw_data, pixel_depth);
        }
        3 => {
            let raw_data = utils::read_n_bytes(&mut reader, raw_data_size)?;
            set_image_data(&mut texture, &raw_data, pixel_depth);
        }
        10 => {
            rle_decode(&mut texture, &mut reader, pixel_depth)?;
            ()
        }
        11 => {
            rle_decode(&mut texture, &mut reader, pixel_depth)?;
        }
        _ => {
            return Err("unsupported file format".into());
        }
    }

    if (tga_header.image_descriptor >> 4) & 1 == 1 {
        texture.flip_horizontally();
    }
    if (tga_header.image_descriptor >> 5) & 1 == 0 {
        texture.flip_vertically();
    }

    return Ok(texture);
}

pub fn write_to_file(texture: &Texture, path: &Path, ignore_alpha: bool) -> utils::Result<()> {
    let file = File::options()
        .write(true)
        .create(true)
        .append(false)
        .open(&path)?;
    let mut writer = BufWriter::new(file);

    let tga_header = TgaHeader {
        image_type: 2,
        image_width: texture.width as u16,
        image_height: texture.height as u16,
        pixel_depth: 32,
        image_descriptor: 0x00,
        ..Default::default()
    };

    unsafe { utils::write_raw_struct(&mut writer, &tga_header)? };

    let mut raw_data = vec![0u8; (texture.width * texture.height * 4) as usize];
    get_image_data(texture, &mut raw_data, ignore_alpha);

    writer.write_all(&raw_data)?;

    Ok(())
}
