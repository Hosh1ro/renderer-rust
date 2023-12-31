use core::str::{FromStr, SplitAsciiWhitespace};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    vec,
};

use crate::{
    math::vector::{Vec2f64, Vec3f64},
    utils,
};

#[derive(Debug)]
pub struct Model {
    pub vertices: Vec<Vec3f64>,
    pub texture_coordinates: Vec<Vec2f64>,
    pub vertex_normals: Vec<Vec3f64>,
    pub face_vertex_indices: Vec<usize>,
    pub face_texture_coordinate_indices: Vec<usize>,
    pub face_vertex_normal_indices: Vec<usize>,
}

impl Model {
    fn parse_vertex(&mut self, iter: &mut SplitAsciiWhitespace) -> utils::Result<()> {
        let raw_data: Vec<f64> = iter
            .map(FromStr::from_str)
            .collect::<Result<Vec<f64>, _>>()?;

        match raw_data.len() {
            3 => {
                self.vertices.push(Vec3f64::new_from_vec(&raw_data));
                Ok(())
            }
            4 => {
                self.vertices.push(Vec3f64::new_from_vec(&raw_data));
                Ok(())
            }
            _ => Err("obj vertex parse fail".into()),
        }
    }

    fn parse_texture_coordinate(&mut self, iter: &mut SplitAsciiWhitespace) -> utils::Result<()> {
        let raw_data: Vec<f64> = iter
            .map(FromStr::from_str)
            .collect::<Result<Vec<f64>, _>>()?;

        match raw_data.len() {
            2 | 3 => {
                let mut uv = Vec2f64::new_from_vec(&raw_data);
                uv[1] = 1f64 - uv[1];
                self.texture_coordinates.push(uv);
                Ok(())
            }
            _ => Err("obj texture coordinate parse fail".into()),
        }
    }

    fn parse_vertex_normal(&mut self, iter: &mut SplitAsciiWhitespace) -> utils::Result<()> {
        let raw_data: Vec<f64> = iter
            .map(FromStr::from_str)
            .collect::<Result<Vec<f64>, _>>()?;

        match raw_data.len() {
            3 => {
                self.vertex_normals.push(Vec3f64::new_from_vec(&raw_data));
                Ok(())
            }
            4 => {
                self.vertex_normals.push(Vec3f64::new_from_vec(&raw_data));
                Ok(())
            }
            _ => Err("obj vertex normal parse fail".into()),
        }
    }

    fn parse_faces(&mut self, iter: &mut SplitAsciiWhitespace) -> utils::Result<()> {
        let raw_data: Vec<Vec<&str>> = iter.map(|s| s.split('/').collect()).collect();

        match raw_data.len() {
            3 => {
                for single_raw_data in &raw_data {
                    match single_raw_data.len() {
                        1 => {
                            self.face_vertex_indices
                                .push(single_raw_data[0].parse::<usize>()? - 1);
                        }
                        2 => {
                            self.face_vertex_indices
                                .push(single_raw_data[0].parse::<usize>()? - 1);
                            self.face_texture_coordinate_indices
                                .push(single_raw_data[1].parse::<usize>()? - 1);
                        }
                        3 => {
                            self.face_vertex_indices
                                .push(single_raw_data[0].parse::<usize>()? - 1);
                            self.face_texture_coordinate_indices
                                .push(single_raw_data[1].parse::<usize>()? - 1);
                            self.face_vertex_normal_indices
                                .push(single_raw_data[2].parse::<usize>()? - 1);
                        }
                        _ => {
                            return Err("obj face parse fail".into());
                        }
                    }
                }
                Ok(())
            }
            _ => Err("obj face parse fail".into()),
        }
    }

    pub fn new() -> Self {
        Model {
            vertices: vec![],
            texture_coordinates: vec![],
            vertex_normals: vec![],
            face_vertex_indices: vec![],
            face_texture_coordinate_indices: vec![],
            face_vertex_normal_indices: vec![],
        }
    }

    pub fn new_from_file(path: &Path) -> utils::Result<Model> {
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut new_model: Model = Model::new();

        let mut buffer = String::new();
        while let Ok(bytes) = reader.read_line(&mut buffer) {
            if bytes == 0 {
                break;
            }
            let mut iter = buffer.split_ascii_whitespace();
            match iter.next() {
                Some("v") => {
                    new_model.parse_vertex(&mut iter)?;
                }
                Some("vt") => {
                    new_model.parse_texture_coordinate(&mut iter)?;
                }
                Some("vn") => {
                    new_model.parse_vertex_normal(&mut iter)?;
                }
                Some("f") => {
                    new_model.parse_faces(&mut iter)?;
                }
                _ => {}
            }
            buffer.clear()
        }

        Ok(new_model)
    }

    pub fn get_nfaces(&self) -> usize {
        self.face_vertex_indices.len() / 3
    }

    pub fn get_uv(&self, face_index: usize, nth_vertex: usize) -> Vec2f64 {
        self.texture_coordinates[self.face_texture_coordinate_indices[face_index * 3 + nth_vertex]]
    }

    pub fn get_normal(&self, face_index: usize, nth_vertex: usize) -> Vec3f64 {
        self.vertex_normals[self.face_vertex_normal_indices[face_index * 3 + nth_vertex]]
    }

    pub fn get_vertex(&self, face_index: usize, nth_vertex: usize) -> Vec3f64 {
        self.vertices[self.face_vertex_indices[face_index * 3 + nth_vertex]]
    }
}
