use crate::{
    math::{matrix::Matrix, vector::Vec2f32, vector::Vec3f32, vector::Vec4f32},
    texture::{Texture, TextureColor},
};

pub trait Shader {
    fn get_model_view(&self) -> &Matrix<f32, 4, 4>;
    fn get_projection(&self) -> &Matrix<f32, 4, 4>;
    fn get_viewport(&self) -> &Matrix<f32, 4, 4>;
    fn vertex(&mut self, face_index: usize, nth_vertex: usize) -> Vec4f32;
    fn fragment(&mut self, barycentric: &Vec3f32, color: &mut TextureColor) -> bool;
    fn run_once(&mut self, zbuffer: &mut Vec<f32>, frame: &mut Texture);
}

pub fn lookat(eye: Vec4f32, center: Vec4f32, up: Vec4f32) -> Matrix<f32, 4, 4> {
    let z = (&center - &eye).normalize();
    let x = z.cross(&up).normalize();
    let y = x.cross(&z).normalize();

    let mut t_view = Matrix::<f32, 4, 4>::new();
    let mut r_view = Matrix::<f32, 4, 4>::new();

    (t_view[0][0], t_view[0][3]) = (1f32, -eye[0]);
    (t_view[1][1], t_view[1][3]) = (1f32, -eye[1]);
    (t_view[2][2], t_view[2][3]) = (1f32, -eye[2]);
    t_view[3][3] = 1f32;

    (r_view[0][0], r_view[0][1], r_view[0][2]) = (x[0], x[1], x[2]);
    (r_view[1][0], r_view[1][1], r_view[1][2]) = (y[0], y[1], y[2]);
    (r_view[2][0], r_view[2][1], r_view[2][2]) = (z[0], z[1], z[2]);
    r_view[3][3] = 1f32;

    &r_view * &t_view
}

pub fn viewport(x: u32, y: u32, w: u32, h: u32) -> Matrix<f32, 4, 4> {
    let (x, y, w, h) = (x as f32, y as f32, w as f32, h as f32);

    let mut viewport = Matrix::<f32, 4, 4>::new();

    (viewport[0][0], viewport[0][3]) = (w / 2f32, w / 2f32 + x);
    (viewport[1][1], viewport[1][3]) = (h / 2f32, h / 2f32 + y);
    viewport[2][2] = 1f32;
    viewport[3][3] = 1f32;

    viewport
}

pub fn projection_pinhole(focal_length: f32) -> Matrix<f32, 4, 4> {
    let mut projection = Matrix::<f32, 4, 4>::new();

    projection[0][0] = 1f32;
    projection[1][1] = 1f32;
    projection[2][2] = 1f32;
    projection[3][2] = 1f32 / focal_length;

    projection
}

#[inline(always)]
pub fn barycentric_coordinates(triangle: &[Vec2f32; 3], point: &Vec2f32) -> Vec3f32 {
    let a = &triangle[0];
    let b = &triangle[1];
    let c = &triangle[2];
    let p = point;

    let (v0, v1, v2) = (b - a, c - a, p - a);
    let mut res = Vec3f32::new();
    let den = v0[0] * v1[1] - v1[0] * v0[1];
    res[1] = (v2[0] * v1[1] - v1[0] * v2[1]) / den;
    res[2] = (v0[0] * v2[1] - v2[0] * v0[1]) / den;
    res[0] = 1.0 - res[1] - res[2];
    res
}

pub fn triangle_rasterize(
    triangle: &[Vec4f32; 3],
    shader: &mut dyn Shader,
    zbuffer: &mut Vec<f32>,
    frame: &mut Texture,
) {
    let screen_triangle: [Vec4f32; 3] = [
        shader.get_viewport() * &triangle[0],
        shader.get_viewport() * &triangle[1],
        shader.get_viewport() * &triangle[2],
    ];
    let screen_triangle_perspective: [Vec2f32; 3] = [
        (&screen_triangle[0] / screen_triangle[0][3]).project(),
        (&screen_triangle[1] / screen_triangle[1][3]).project(),
        (&screen_triangle[2] / screen_triangle[2][3]).project(),
    ];

    let mut bbox_min = [
        (frame.get_width() - 1) as f32,
        (frame.get_height() - 1) as f32,
    ];
    let mut bbox_max = [0f32; 2];
    for vertex in screen_triangle_perspective {
        bbox_min[0] = if bbox_min[0] < vertex[0].floor() {
            bbox_min[0]
        } else {
            vertex[0].floor()
        };
        bbox_min[1] = if bbox_min[1] < vertex[1].floor() {
            bbox_min[1]
        } else {
            vertex[1].floor()
        };
        bbox_max[0] = if bbox_max[0] > vertex[0].ceil() {
            bbox_max[0]
        } else {
            vertex[0].ceil()
        };
        bbox_max[1] = if bbox_max[1] > vertex[1].ceil() {
            bbox_max[1]
        } else {
            vertex[1].ceil()
        };
    }
    if bbox_min[0] < 0f32 {
        bbox_min[0] = 0f32
    }
    if bbox_min[1] < 0f32 {
        bbox_min[1] = 0f32
    }
    if bbox_max[0] > (frame.get_width() - 1) as f32 {
        bbox_max[0] = (frame.get_width() - 1) as f32
    }
    if bbox_max[1] > (frame.get_height() - 1) as f32 {
        bbox_max[1] = (frame.get_height() - 1) as f32
    }

    let bbox_min: [u32; 2] = unsafe {
        [
            bbox_min[0].to_int_unchecked(),
            bbox_min[1].to_int_unchecked(),
        ]
    };
    let bbox_max: [u32; 2] = unsafe {
        [
            bbox_max[0].to_int_unchecked(),
            bbox_max[1].to_int_unchecked(),
        ]
    };

    for x in bbox_min[0]..=bbox_max[0] {
        for y in bbox_min[1]..=bbox_max[1] {
            // barycentric interpolation and perspective correct
            let screen_barycentric = barycentric_coordinates(
                &screen_triangle_perspective,
                &Vec2f32::new_from_vec(&vec![x as f32, y as f32]),
            );
            let mut clip_barycentric = Vec3f32::new_from_vec(&vec![
                screen_barycentric[0] / screen_triangle[0][3],
                screen_barycentric[1] / screen_triangle[1][3],
                screen_barycentric[2] / screen_triangle[2][3],
            ]);
            clip_barycentric = &clip_barycentric
                / (clip_barycentric[0] + clip_barycentric[1] + clip_barycentric[2]);

            // interpolate depth
            let fragment_depth =
                &Vec3f32::new_from_vec(&vec![triangle[0][2], triangle[1][2], triangle[2][2]])
                    * &clip_barycentric;
            let fragment_index = (x + y * frame.get_width()) as usize;

            if screen_barycentric[0] < 0f32
                || screen_barycentric[1] < 0f32
                || screen_barycentric[2] < 0f32
                || fragment_depth > zbuffer[fragment_index]
            {
                continue;
            }

            let mut color: TextureColor = TextureColor {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            };
            if shader.fragment(&clip_barycentric, &mut color) {
                zbuffer[fragment_index] = fragment_depth;
                frame.set_color(x, y, color).unwrap();
            }
        }
    }
}
