use std::{cmp, path::Path, vec};

use librender::{
    math::{
        matrix::Matrix,
        vector::{Vec2f64, Vec3f64, Vec4f64, Vector},
    },
    model::{self, Model},
    render::{self, Shader},
    texture::{self, Texture, TextureColor},
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const WIDTH_SHADOW: u32 = 800;
const HEIGHT_SHADOW: u32 = 800;

struct AShader<'a> {
    model_view: Matrix<f64, 4, 4>,
    projection: Matrix<f64, 4, 4>,
    viewport: Matrix<f64, 4, 4>,
    model_view_inv_t: Matrix<f64, 4, 4>,
    uv: Matrix<f64, 2, 3>,
    normal: Matrix<f64, 3, 3>,
    view: Matrix<f64, 3, 3>,
    light: Vec3f64,
    normal_map: Option<&'a Texture>,
    specular_map: Option<&'a Texture>,
    diffuse_map: Option<&'a Texture>,
    model: Option<&'a Model>,
    shadow_zbuffer: Option<&'a Vec<f64>>,
    shadow_matrix: Option<Matrix<f64, 4, 4>>,
}

impl<'a> AShader<'a> {
    pub fn new(
        model_view: Matrix<f64, 4, 4>,
        projection: Matrix<f64, 4, 4>,
        viewport: Matrix<f64, 4, 4>,
        light: Vec3f64,
    ) -> AShader<'a> {
        AShader {
            model_view,
            projection,
            viewport,
            model_view_inv_t: model_view.inv().transpose(),
            uv: Matrix::new(),
            normal: Matrix::new(),
            view: Matrix::new(),
            light: (&model_view * &light.embed::<4>(0f64))
                .project::<3>()
                .normalize(),
            normal_map: None,
            specular_map: None,
            diffuse_map: None,
            model: None,
            shadow_zbuffer: None,
            shadow_matrix: None,
        }
    }

    pub fn set_normal_map(&mut self, normal_map: Option<&'a Texture>) {
        self.normal_map = normal_map
    }

    pub fn set_specular_map(&mut self, specular_map: Option<&'a Texture>) {
        self.specular_map = specular_map
    }

    pub fn set_diffuse_map(&mut self, diffuse_map: Option<&'a Texture>) {
        self.diffuse_map = diffuse_map
    }

    pub fn set_model(&mut self, model: &'a Model) {
        self.model = Some(model)
    }

    pub fn set_shadow_zbuffer(&mut self, buffer: &'a Vec<f64>) {
        self.shadow_zbuffer = Some(buffer)
    }

    pub fn set_shadow_matrix(&mut self, matrix: Matrix<f64, 4, 4>) {
        self.shadow_matrix = Some(matrix)
    }

    fn get_normal(&self, uv: &Vec2f64) -> Vec3f64 {
        let color = self.normal_map.unwrap().sample(uv);
        &(&(&Vec3f64::new_from_vec(&vec![color.r as f64, color.g as f64, color.b as f64]) * 2f64)
            / 255f64)
            - &Vec3f64::new_from_vec(&vec![1f64, 1f64, 1f64])
    }
}

impl<'a> Shader for AShader<'a> {
    fn get_model_view(&self) -> &Matrix<f64, 4, 4> {
        &self.model_view
    }

    fn get_projection(&self) -> &Matrix<f64, 4, 4> {
        &self.projection
    }

    fn get_viewport(&self) -> &Matrix<f64, 4, 4> {
        &self.viewport
    }

    fn vertex(&mut self, face_index: usize, nth_vertex: usize) -> Vec4f64 {
        self.uv.set_col(
            nth_vertex,
            &self.model.unwrap().get_uv(face_index, nth_vertex),
        );
        self.normal.set_col(
            nth_vertex,
            &(&self.model_view_inv_t
                * &self
                    .model
                    .unwrap()
                    .get_normal(face_index, nth_vertex)
                    .embed::<4>(0f64))
                .project::<3>(),
        );
        let view_space = &self.model_view
            * &self
                .model
                .unwrap()
                .get_vertex(face_index, nth_vertex)
                .embed::<4>(1f64);
        self.view.set_col(nth_vertex, &view_space.project::<3>());
        &self.projection * &view_space
    }

    fn fragment(&mut self, barycentric: &Vec3f64, color: &mut TextureColor) -> bool {
        let normal_inter = (&self.normal * barycentric).normalize();
        let uv_inter = &self.uv * barycentric;

        let mut a_i: Matrix<f64, 3, 3> = Matrix::new();
        a_i[0] = &self.view.get_col(1) - &self.view.get_col(0);
        a_i[1] = &self.view.get_col(2) - &self.view.get_col(0);
        a_i[2] = normal_inter;
        a_i = a_i.inv();
        let i = &a_i
            * &Vector::<f64, 3>::new_from_vec(&vec![
                self.uv[0][1] - self.uv[0][0],
                self.uv[0][2] - self.uv[0][0],
                0f64,
            ]);
        let j = &a_i
            * &Vector::<f64, 3>::new_from_vec(&vec![
                self.uv[1][1] - self.uv[1][0],
                self.uv[1][2] - self.uv[1][0],
                0f64,
            ]);
        let mut b: Matrix<f64, 3, 3> = Matrix::new();
        b[0] = i.normalize();
        b[1] = j.normalize();
        b[2] = normal_inter;
        b = b.transpose();

        let mut shadow = 1.0;
        if let Some(_) = self.shadow_zbuffer {
            let shadow_mapping_pos =
                &self.shadow_matrix.unwrap() * &(&self.view * barycentric).embed::<4>(1f64);
            let depth = shadow_mapping_pos[2];
            let shadow_mapping_pos = &shadow_mapping_pos / shadow_mapping_pos[3];

            if shadow_mapping_pos[0] < 0f64
                || shadow_mapping_pos[0] > WIDTH_SHADOW as f64
                || shadow_mapping_pos[1] < 0f64
                || shadow_mapping_pos[1] > HEIGHT_SHADOW as f64
            {
                shadow = 1.0;
            } else {
                let x: i32 = unsafe { shadow_mapping_pos[0].floor().to_int_unchecked() };
                let y: i32 = unsafe { shadow_mapping_pos[1].floor().to_int_unchecked() };

                let index = x + y * WIDTH_SHADOW as i32;
                if depth - self.shadow_zbuffer.unwrap()[index as usize] > 1e-1 {
                    shadow = 0.3;
                }
            }
        }

        let n = (&b * &self.get_normal(&uv_inter)).normalize();
        let mut diffuse = &n * &self.light;
        if diffuse < 0f64 {
            diffuse = 0f64
        }
        let reflection = (&(&n * ((&n * &self.light) * 2f64)) - &self.light).normalize();
        let specular = if let Some(_) = self.specular_map {
            if -reflection[2] > 0f64 {
                f64::powf(
                    -reflection[2],
                    5f64 + self.specular_map.unwrap().sample(&uv_inter).b as f64,
                )
            } else {
                0f64
            }
        } else {
            0f64
        };

        let diffuse_color = self.diffuse_map.unwrap().sample(&uv_inter);
        color.r = cmp::min::<u32>(
            10 + (diffuse_color.r as f64 * shadow * (diffuse + specular)) as u32,
            255,
        ) as u8;
        color.g = cmp::min::<u32>(
            10 + (diffuse_color.g as f64 * shadow * (diffuse + specular)) as u32,
            255,
        ) as u8;
        color.b = cmp::min::<u32>(
            10 + (diffuse_color.b as f64 * shadow * (diffuse + specular)) as u32,
            255,
        ) as u8;
        color.a = 255u8;

        true
    }

    fn run_once(&mut self, zbuffer: &mut Vec<f64>, frame: &mut Texture) {
        for i in 0..self.model.unwrap().get_nfaces() {
            let mut clip_triangle = [Vector::<f64, 4>::new(); 3];

            for j in 0..3 {
                clip_triangle[j] = self.vertex(i, j);
            }

            render::triangle_rasterize(&clip_triangle, self, zbuffer, frame);
        }
    }
}

struct ShadowShader<'a> {
    model_view: Matrix<f64, 4, 4>,
    projection: Matrix<f64, 4, 4>,
    viewport: Matrix<f64, 4, 4>,
    model_view_projection: Matrix<f64, 4, 4>,
    model: Option<&'a Model>,
}

impl<'a> ShadowShader<'a> {
    pub fn new(
        model_view: Matrix<f64, 4, 4>,
        projection: Matrix<f64, 4, 4>,
        viewport: Matrix<f64, 4, 4>,
    ) -> ShadowShader<'a> {
        ShadowShader {
            model_view,
            projection,
            viewport,
            model_view_projection: &projection * &model_view,
            model: None,
        }
    }

    pub fn set_model(&mut self, model: &'a Model) {
        self.model = Some(model)
    }
}

impl<'a> Shader for ShadowShader<'a> {
    fn get_model_view(&self) -> &Matrix<f64, 4, 4> {
        &self.model_view
    }

    fn get_projection(&self) -> &Matrix<f64, 4, 4> {
        &self.projection
    }

    fn get_viewport(&self) -> &Matrix<f64, 4, 4> {
        &self.viewport
    }

    fn vertex(&mut self, face_index: usize, nth_vertex: usize) -> Vec4f64 {
        &self.model_view_projection
            * &self
                .model
                .unwrap()
                .get_vertex(face_index, nth_vertex)
                .embed::<4>(1f64)
    }

    fn fragment(&mut self, _barycentric: &Vec3f64, _color: &mut TextureColor) -> bool {
        true
    }

    fn run_once(&mut self, zbuffer: &mut Vec<f64>, frame: &mut Texture) {
        for i in 0..self.model.unwrap().get_nfaces() {
            let mut clip_triangle = [Vector::<f64, 4>::new(); 3];

            for j in 0..3 {
                clip_triangle[j] = self.vertex(i, j);
            }

            render::triangle_rasterize(&clip_triangle, self, zbuffer, frame);
        }
    }
}

fn main() {
    let mut frame = texture::Texture::new(WIDTH, HEIGHT);
    let mut shadow_frame = texture::Texture::new(WIDTH_SHADOW, HEIGHT_SHADOW);
    let mut zbuffer = vec![f64::MAX; (WIDTH * HEIGHT) as usize];
    let mut shadow_zbuffer = vec![f64::MAX; (WIDTH_SHADOW * HEIGHT_SHADOW) as usize];

    let eye: Vec3f64 = Vector::new_from_vec(&vec![1f64, 1f64, 3f64]);
    let center: Vec3f64 = Vector::new_from_vec(&vec![0f64, 0f64, 0f64]);
    let up: Vec3f64 = Vector::new_from_vec(&vec![0f64, 1f64, 0f64]);
    let light: Vec3f64 = Vector::new_from_vec(&vec![1f64, 2f64, 1f64]);

    let model_view: Matrix<f64, 4, 4> = render::lookat(eye, center, up);
    let projection: Matrix<f64, 4, 4> = render::projection_pinhole((&center - &eye).norm_l2());
    let viewport: Matrix<f64, 4, 4> =
        render::viewport(WIDTH / 8, HEIGHT / 8, WIDTH * 3 / 4, HEIGHT * 3 / 4);
    let mut shader = AShader::new(model_view, projection, viewport, light);

    let model_view_light: Matrix<f64, 4, 4> = render::lookat(light, center, up);
    let projection_light: Matrix<f64, 4, 4> =
        render::projection_pinhole((&center - &light).norm_l2());
    let viewport_light: Matrix<f64, 4, 4> = render::viewport(
        WIDTH_SHADOW / 8,
        HEIGHT_SHADOW / 8,
        WIDTH_SHADOW * 3 / 4,
        HEIGHT_SHADOW * 3 / 4,
    );
    let mut shadow_shader = ShadowShader::new(model_view_light, projection_light, viewport_light);

    let model_floor = model::Model::new_from_file(Path::new("obj/floor.obj")).unwrap();
    let normal_map_floor =
        texture::tga::read_from_file(Path::new("obj/floor_nm_tangent.tga")).unwrap();
    let diffuse_map_floor =
        texture::tga::read_from_file(Path::new("obj/floor_diffuse.tga")).unwrap();

    let model =
        model::Model::new_from_file(Path::new("obj/diablo3_pose/diablo3_pose.obj")).unwrap();
    let normal_map =
        texture::tga::read_from_file(Path::new("obj/diablo3_pose/diablo3_pose_nm_tangent.tga"))
            .unwrap();
    let diffuse_map =
        texture::tga::read_from_file(Path::new("obj/diablo3_pose/diablo3_pose_diffuse.tga"))
            .unwrap();
    let specular_map =
        texture::tga::read_from_file(Path::new("obj/diablo3_pose/diablo3_pose_spec.tga")).unwrap();

    shadow_shader.set_model(&model_floor);
    shadow_shader.run_once(&mut shadow_zbuffer, &mut shadow_frame);
    shadow_shader.set_model(&model);
    shadow_shader.run_once(&mut shadow_zbuffer, &mut shadow_frame);

    shader.set_shadow_zbuffer(&shadow_zbuffer);
    shader.set_shadow_matrix(
        &(&(&viewport_light * &projection_light) * &model_view_light) * &model_view.inv(),
    );

    shader.set_model(&model_floor);
    shader.set_normal_map(Some(&normal_map_floor));
    shader.set_diffuse_map(Some(&diffuse_map_floor));
    shader.set_specular_map(None);
    shader.run_once(&mut zbuffer, &mut frame);

    shader.set_model(&model);
    shader.set_normal_map(Some(&normal_map));
    shader.set_diffuse_map(Some(&diffuse_map));
    shader.set_specular_map(Some(&specular_map));
    shader.run_once(&mut zbuffer, &mut frame);

    texture::tga::write_to_file(&frame, Path::new("result.tga"), true).unwrap();
}
