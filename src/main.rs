use std::{
    cmp::{self, min},
    f32::consts::PI,
    path::Path,
    vec,
};

use librender::{
    math::{
        matrix::Matrix,
        vector::{Vec2f32, Vec3f32, Vec4f32, Vector},
    },
    model::{self, Model},
    render::{self, Shader},
    texture::{self, Texture, TextureColor},
};

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1600;

const WIDTH_SHADOW: u32 = 1600;
const HEIGHT_SHADOW: u32 = 1600;

struct AShader<'a> {
    model_view: Matrix<f32, 4, 4>,
    projection: Matrix<f32, 4, 4>,
    viewport: Matrix<f32, 4, 4>,
    model_view_inv_t: Matrix<f32, 4, 4>,
    uv: Matrix<f32, 2, 3>,
    normal: Matrix<f32, 4, 4>,
    view: Matrix<f32, 4, 4>,
    light: Vec4f32,
    normal_map: Option<&'a Texture>,
    specular_map: Option<&'a Texture>,
    diffuse_map: Option<&'a Texture>,
    model: Option<&'a Model>,
    shadow_zbuffer: Option<&'a Vec<f32>>,
    shadow_matrix: Option<Matrix<f32, 4, 4>>,
}

impl<'a> AShader<'a> {
    pub fn new(
        model_view: Matrix<f32, 4, 4>,
        projection: Matrix<f32, 4, 4>,
        viewport: Matrix<f32, 4, 4>,
        light: Vec4f32,
    ) -> AShader<'a> {
        AShader {
            model_view,
            projection,
            viewport,
            model_view_inv_t: model_view.inv().transpose(),
            uv: Matrix::new(),
            normal: Matrix::new(),
            view: Matrix::new(),
            light: (&model_view * &light).normalize(),
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

    pub fn set_shadow_zbuffer(&mut self, buffer: &'a Vec<f32>) {
        self.shadow_zbuffer = Some(buffer)
    }

    pub fn set_shadow_matrix(&mut self, matrix: Matrix<f32, 4, 4>) {
        self.shadow_matrix = Some(matrix)
    }

    fn get_normal(&self, uv: &Vec2f32) -> Vec3f32 {
        let color = self.normal_map.unwrap().sample(uv);
        &(&(&Vec3f32::new_from_vec(&vec![color.r as f32, color.g as f32, color.b as f32]) * 2f32)
            / 255f32)
            - &Vec3f32::new_from_vec(&vec![1f32, 1f32, 1f32])
    }

    fn max_horizon_angle(&self, zbuffer: &Vec<f32>, point: Vec2f32, dir: Vec2f32) -> f32 {
        let point_z = zbuffer[(point[0] as u32 + point[1] as u32 * WIDTH) as usize];

        let mut res = 0f32;
        let mut step = 0f32;
        let mut max_dis = 0f32;
        while step < 100f32 {
            let sample_point = &point + &(&dir * step);
            let sample_point_x = sample_point[0] as i32;
            let sample_point_y = sample_point[1] as i32;

            if sample_point_x < 0
                || sample_point_x as u32 >= WIDTH
                || sample_point_y < 0
                || sample_point_y as u32 >= HEIGHT
            {
                break;
            }

            let dis = (&sample_point - &point).norm_l2();
            if dis < 1f32 {
                step = step + 1.0;
                continue;
            }
            let (sample_point_x, sample_point_y) = (sample_point_x as u32, sample_point_y as u32);
            let sample_point_z = zbuffer[(sample_point_x + sample_point_y * WIDTH) as usize];
            let sample_angle = ((point_z - sample_point_z) / dis).atan();
            if point_z - sample_point_z > 1e-1 {
                step = step + 1.0;
                continue;
            }

            if sample_angle > res {
                res = sample_angle;
                max_dis = dis;
            }

            step = step + 1.0;
        }

        res * (1f32 - max_dis / 10f32)
    }

    fn ao(&self, zbuffer: &Vec<f32>, frame: &mut Texture) {
        for x in 0..frame.get_width() {
            for y in 0..frame.get_height() {
                if zbuffer[(x + y * frame.get_width()) as usize] > 1e5 {
                    continue;
                }

                let mut ao = 0f32;
                let mut alpha = 0f32;

                let point = Vec2f32::new_from_vec(&vec![x as f32, y as f32]);
                for _ in 0..8 {
                    let dir = Vec2f32::new_from_vec(&vec![alpha.cos(), alpha.sin()]);
                    ao += PI / 2.0 - self.max_horizon_angle(&zbuffer, point, dir);
                    alpha += PI / 4.0;
                }

                ao /= (PI / 2.0) * 8.0;
                ao = ao.powf(100f32);
                let mut color = frame.get_color(x, y).unwrap();
                color.r = (color.r as f32 * ao) as u8;
                color.g = (color.g as f32 * ao) as u8;
                color.b = (color.b as f32 * ao) as u8;
                frame.set_color(x, y, color).unwrap();
            }
        }
    }

    fn ssaa(&self, frame: &Texture) -> Texture {
        let mut new_frame = texture::Texture::new(frame.get_width() / 2, frame.get_height() / 2);

        for x in 0..new_frame.get_width() {
            for y in 0..new_frame.get_height() {
                let (x_up, y_up) = (2 * x as i32, 2 * y as i32);
                let mut cnt = 0u32;
                let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);

                for i in -1i32..=1 {
                    for j in -1i32..=1 {
                        let (nx, ny) = (x_up + i, y_up + j);
                        if nx < 0
                            || nx >= frame.get_width() as i32
                            || ny < 0
                            || ny >= frame.get_height() as i32
                        {
                            continue;
                        }

                        r += frame.get_color(nx as u32, ny as u32).unwrap().r as u32;
                        g += frame.get_color(nx as u32, ny as u32).unwrap().g as u32;
                        b += frame.get_color(nx as u32, ny as u32).unwrap().b as u32;
                        cnt += 1;
                    }
                }

                let r = min(r / cnt, 255u32) as u8;
                let g = min(g / cnt, 255u32) as u8;
                let b = min(b / cnt, 255u32) as u8;
                new_frame
                    .set_color(x, y, TextureColor { r, g, b, a: 255 })
                    .unwrap();
            }
        }

        new_frame
    }
}

impl<'a> Shader for AShader<'a> {
    fn get_model_view(&self) -> &Matrix<f32, 4, 4> {
        &self.model_view
    }

    fn get_projection(&self) -> &Matrix<f32, 4, 4> {
        &self.projection
    }

    fn get_viewport(&self) -> &Matrix<f32, 4, 4> {
        &self.viewport
    }

    fn vertex(&mut self, face_index: usize, nth_vertex: usize) -> Vec4f32 {
        self.uv.set_col(
            nth_vertex,
            &self.model.unwrap().get_uv(face_index, nth_vertex),
        );
        self.normal.set_col(
            nth_vertex,
            &(&self.model_view_inv_t * &self.model.unwrap().get_normal(face_index, nth_vertex))
                .project::<3>()
                .embed(0f32),
        );
        let view_space = &self.model_view * &self.model.unwrap().get_vertex(face_index, nth_vertex);
        self.view.set_col(nth_vertex, &view_space);
        &self.projection * &view_space
    }

    fn fragment(&mut self, barycentric: &Vec3f32, color: &mut TextureColor) -> bool {
        let barycentric_homo = barycentric.embed::<4>(0f32);
        let normal_inter = (&self.normal * &barycentric_homo).normalize();
        let uv_inter =
            Vec2f32::new_from_array([&self.uv[0] * barycentric, &self.uv[1] * barycentric]);

        let mut a: Matrix<f32, 4, 4> = Matrix::new();
        a[0] = &self.view.get_col(1) - &self.view.get_col(0);
        a[1] = &self.view.get_col(2) - &self.view.get_col(0);
        a[2] = normal_inter;
        a[3] = Vec4f32::new_from_vec(&vec![0f32, 0f32, 0f32, 1f32]);
        let a_inv = a.inv();
        let u = &a_inv
            * &Vec4f32::new_from_vec(&vec![
                self.uv[0][1] - self.uv[0][0],
                self.uv[0][2] - self.uv[0][0],
                0f32,
                0f32,
            ]);
        let v = &a_inv
            * &Vec4f32::new_from_vec(&vec![
                self.uv[1][1] - self.uv[1][0],
                self.uv[1][2] - self.uv[1][0],
                0f32,
                0f32,
            ]);
        let mut b: Matrix<f32, 4, 4> = Matrix::new();
        b[0] = u.normalize();
        b[1] = v.normalize();
        b[2] = normal_inter;
        b[3] = Vec4f32::new_from_vec(&vec![0f32, 0f32, 0f32, 1f32]);
        b = b.transpose();

        let mut shadow = 1.0;
        if let Some(_) = self.shadow_zbuffer {
            let shadow_mapping_pos =
                &self.shadow_matrix.unwrap() * &(&self.view * &barycentric_homo);
            let depth = shadow_mapping_pos[2];
            let shadow_mapping_pos = &shadow_mapping_pos / shadow_mapping_pos[3];

            if shadow_mapping_pos[0] < 0f32
                || shadow_mapping_pos[0] > WIDTH_SHADOW as f32
                || shadow_mapping_pos[1] < 0f32
                || shadow_mapping_pos[1] > HEIGHT_SHADOW as f32
            {
                shadow = 1.0;
            } else {
                let x: i32 = unsafe { shadow_mapping_pos[0].floor().to_int_unchecked() };
                let y: i32 = unsafe { shadow_mapping_pos[1].floor().to_int_unchecked() };

                let index = x + y * WIDTH_SHADOW as i32;
                if depth - self.shadow_zbuffer.unwrap()[index as usize] > 1e-1 {
                    shadow = 0.4;
                }
            }
        }

        let n = (&b * &self.get_normal(&uv_inter).embed(0f32)).normalize();
        let mut diffuse = &n * &self.light;
        if diffuse < 0f32 {
            diffuse = 0f32
        }
        let reflection = (&(&n * ((&n * &self.light) * 2f32)) - &self.light).normalize();
        let specular = if let Some(_) = self.specular_map {
            if -reflection[2] > 0f32 {
                f32::powf(
                    -reflection[2],
                    5f32 + self.specular_map.unwrap().sample(&uv_inter).b as f32,
                )
            } else {
                0f32
            }
        } else {
            0f32
        };

        let diffuse_color = self.diffuse_map.unwrap().sample(&uv_inter);
        color.r = cmp::min::<u32>(
            20 + (diffuse_color.r as f32 * shadow * (diffuse + specular)) as u32,
            255,
        ) as u8;
        color.g = cmp::min::<u32>(
            20 + (diffuse_color.g as f32 * shadow * (diffuse + specular)) as u32,
            255,
        ) as u8;
        color.b = cmp::min::<u32>(
            20 + (diffuse_color.b as f32 * shadow * (diffuse + specular)) as u32,
            255,
        ) as u8;
        color.a = 255u8;

        true
    }

    fn run_once(&mut self, zbuffer: &mut Vec<f32>, frame: &mut Texture) {
        for i in 0..self.model.unwrap().get_nfaces() {
            let mut clip_triangle = [Vector::<f32, 4>::new(); 3];

            for j in 0..3 {
                clip_triangle[j] = self.vertex(i, j);
            }

            render::triangle_rasterize(&clip_triangle, self, zbuffer, frame);
        }
    }
}

struct ShadowShader<'a> {
    model_view: Matrix<f32, 4, 4>,
    projection: Matrix<f32, 4, 4>,
    viewport: Matrix<f32, 4, 4>,
    model_view_projection: Matrix<f32, 4, 4>,
    model: Option<&'a Model>,
}

impl<'a> ShadowShader<'a> {
    pub fn new(
        model_view: Matrix<f32, 4, 4>,
        projection: Matrix<f32, 4, 4>,
        viewport: Matrix<f32, 4, 4>,
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
    fn get_model_view(&self) -> &Matrix<f32, 4, 4> {
        &self.model_view
    }

    fn get_projection(&self) -> &Matrix<f32, 4, 4> {
        &self.projection
    }

    fn get_viewport(&self) -> &Matrix<f32, 4, 4> {
        &self.viewport
    }

    fn vertex(&mut self, face_index: usize, nth_vertex: usize) -> Vec4f32 {
        &self.model_view_projection * &self.model.unwrap().get_vertex(face_index, nth_vertex)
    }

    fn fragment(&mut self, _barycentric: &Vec3f32, _color: &mut TextureColor) -> bool {
        true
    }

    fn run_once(&mut self, zbuffer: &mut Vec<f32>, frame: &mut Texture) {
        for i in 0..self.model.unwrap().get_nfaces() {
            let mut clip_triangle = [Vector::<f32, 4>::new(); 3];

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
    let mut zbuffer = vec![f32::MAX; (WIDTH * HEIGHT) as usize];
    let mut shadow_zbuffer = vec![f32::MAX; (WIDTH_SHADOW * HEIGHT_SHADOW) as usize];

    let eye: Vec4f32 = Vector::new_from_vec(&vec![1f32, 1f32, 3f32, 1f32]);
    let center: Vec4f32 = Vector::new_from_vec(&vec![0f32, 0f32, 0f32, 1f32]);
    let up: Vec4f32 = Vector::new_from_vec(&vec![0f32, 1f32, 0f32, 0f32]);
    let light: Vec4f32 = Vector::new_from_vec(&vec![1f32, 2f32, 1f32, 1f32]);

    let model_view: Matrix<f32, 4, 4> = render::lookat(eye, center, up);
    let projection: Matrix<f32, 4, 4> = render::projection_pinhole((&center - &eye).norm_l2());
    let viewport: Matrix<f32, 4, 4> =
        render::viewport(WIDTH / 8, HEIGHT / 8, WIDTH * 3 / 4, HEIGHT * 3 / 4);
    let mut shader = AShader::new(model_view, projection, viewport, &light - &center);

    let model_view_light: Matrix<f32, 4, 4> = render::lookat(light, center, up);
    let projection_light: Matrix<f32, 4, 4> =
        render::projection_pinhole((&center - &light).norm_l2());
    let viewport_light: Matrix<f32, 4, 4> = render::viewport(
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

    // let body_model = model::Model::new_from_file(Path::new("obj/boggie/body.obj")).unwrap();
    // let body_normal_map =
    //     texture::tga::read_from_file(Path::new("obj/boggie/body_nm_tangent.tga")).unwrap();
    // let body_diffuse_map =
    //     texture::tga::read_from_file(Path::new("obj/boggie/body_diffuse.tga")).unwrap();
    // let body_specular_map =
    //     texture::tga::read_from_file(Path::new("obj/boggie/body_spec.tga")).unwrap();

    // let eyes_model = model::Model::new_from_file(Path::new("obj/boggie/eyes.obj")).unwrap();
    // let eyes_normal_map =
    //     texture::tga::read_from_file(Path::new("obj/boggie/eyes_nm_tangent.tga")).unwrap();
    // let eyes_diffuse_map =
    //     texture::tga::read_from_file(Path::new("obj/boggie/eyes_diffuse.tga")).unwrap();
    // let eyes_specular_map =
    //     texture::tga::read_from_file(Path::new("obj/boggie/eyes_spec.tga")).unwrap();

    // let head_model = model::Model::new_from_file(Path::new("obj/boggie/head.obj")).unwrap();
    // let head_normal_map =
    //     texture::tga::read_from_file(Path::new("obj/boggie/head_nm_tangent.tga")).unwrap();
    // let head_diffuse_map =
    //     texture::tga::read_from_file(Path::new("obj/boggie/head_diffuse.tga")).unwrap();
    // let head_specular_map =
    //     texture::tga::read_from_file(Path::new("obj/boggie/head_spec.tga")).unwrap();

    shadow_shader.set_model(&model_floor);
    shadow_shader.run_once(&mut shadow_zbuffer, &mut shadow_frame);
    shadow_shader.set_model(&model);
    shadow_shader.run_once(&mut shadow_zbuffer, &mut shadow_frame);
    // shadow_shader.set_model(&body_model);
    // shadow_shader.run_once(&mut shadow_zbuffer, &mut shadow_frame);
    // shadow_shader.set_model(&eyes_model);
    // shadow_shader.run_once(&mut shadow_zbuffer, &mut shadow_frame);
    // shadow_shader.set_model(&head_model);
    // shadow_shader.run_once(&mut shadow_zbuffer, &mut shadow_frame);

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

    // shader.set_model(&body_model);
    // shader.set_normal_map(Some(&body_normal_map));
    // shader.set_diffuse_map(Some(&body_diffuse_map));
    // shader.set_specular_map(Some(&body_specular_map));
    // shader.run_once(&mut zbuffer, &mut frame);

    // shader.set_model(&eyes_model);
    // shader.set_normal_map(Some(&eyes_normal_map));
    // shader.set_diffuse_map(Some(&eyes_diffuse_map));
    // shader.set_specular_map(Some(&eyes_specular_map));
    // shader.run_once(&mut zbuffer, &mut frame);

    // shader.set_model(&head_model);
    // shader.set_normal_map(Some(&head_normal_map));
    // shader.set_diffuse_map(Some(&head_diffuse_map));
    // shader.set_specular_map(Some(&head_specular_map));
    // shader.run_once(&mut zbuffer, &mut frame);

    shader.ao(&zbuffer, &mut frame);

    let frame = shader.ssaa(&frame);

    texture::tga::write_to_file(&frame, Path::new("result.tga"), true).unwrap();
}
