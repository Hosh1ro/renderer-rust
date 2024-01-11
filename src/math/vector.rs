use std::{
    fmt::Debug,
    ops,
    simd::{prelude::SimdFloat, Simd, SimdElement},
};

#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const N: usize>([T; N])
where
    T: Default + Debug + Copy + SimdElement;

pub type Vec2f32 = Vector<f32, 2>;
pub type Vec2f64 = Vector<f64, 2>;
pub type Vec3f32 = Vector<f32, 3>;
pub type Vec3f64 = Vector<f64, 3>;
pub type Vec4f32 = Vector<f32, 4>;
pub type Vec4f64 = Vector<f64, 4>;

impl ops::Add for &Vec4f32 {
    type Output = Vec4f32;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let (a, b) = (Simd::from(self.0), Simd::from(rhs.0));
        Vec4f32::new_from_array((a + b).to_array())
    }
}

impl ops::Sub for &Vec4f32 {
    type Output = Vec4f32;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let (a, b) = (Simd::from(self.0), Simd::from(rhs.0));
        Vec4f32::new_from_array((a - b).to_array())
    }
}

impl ops::Mul for &Vec4f32 {
    type Output = f32;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        let (a, b) = (Simd::from(self.0), Simd::from(rhs.0));
        Simd::from(a * b).reduce_sum()
    }
}

impl ops::Mul<f32> for &Vec4f32 {
    type Output = Vec4f32;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        let (a, b) = (Simd::from(self.0), Simd::splat(rhs));
        Vec4f32::new_from_array((a * b).to_array())
    }
}

impl ops::Div<f32> for &Vec4f32 {
    type Output = Vec4f32;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        let (a, b) = (Simd::from(self.0), Simd::splat(rhs));
        Vec4f32::new_from_array((a / b).to_array())
    }
}

impl Vec4f32 {
    #[inline(always)]
    pub fn norm_l2(&self) -> f32 {
        (self * self).sqrt()
    }

    #[inline(always)]
    pub fn normalize(&self) -> Self {
        self / self.norm_l2()
    }

    #[inline(always)]
    pub fn cross(&self, rhs: &Self) -> Self {
        let a = self;
        let b = rhs;
        Self([
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
            0f32,
        ])
    }
}

impl ops::Add for &Vec2f32 {
    type Output = Vec2f32;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let (a, b) = (Simd::from(self.0), Simd::from(rhs.0));
        Vec2f32::new_from_array((a + b).to_array())
    }
}

impl ops::Sub for &Vec2f32 {
    type Output = Vec2f32;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let (a, b) = (Simd::from(self.0), Simd::from(rhs.0));
        Vec2f32::new_from_array((a - b).to_array())
    }
}

impl ops::Mul for &Vec2f32 {
    type Output = f32;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        let (a, b) = (Simd::from(self.0), Simd::from(rhs.0));
        Simd::from(a * b).reduce_sum()
    }
}

impl ops::Mul<f32> for &Vec2f32 {
    type Output = Vec2f32;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        let (a, b) = (Simd::from(self.0), Simd::splat(rhs));
        Vec2f32::new_from_array((a * b).to_array())
    }
}

impl ops::Div<f32> for &Vec2f32 {
    type Output = Vec2f32;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        let (a, b) = (Simd::from(self.0), Simd::splat(rhs));
        Vec2f32::new_from_array((a / b).to_array())
    }
}

impl Vec2f32 {
    #[inline(always)]
    pub fn norm_l2(&self) -> f32 {
        (self * self).sqrt()
    }

    #[inline(always)]
    pub fn normalize(&self) -> Self {
        self / self.norm_l2()
    }
}

impl ops::Add for &Vec3f32 {
    type Output = Vec3f32;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let (a, b) = (
            Simd::from(self.embed::<4>(0f32).0),
            Simd::from(rhs.embed::<4>(0f32).0),
        );
        Vec3f32::new_from_vec(&(a + b).to_array().to_vec())
    }
}

impl ops::Sub for &Vec3f32 {
    type Output = Vec3f32;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let (a, b) = (
            Simd::from(self.embed::<4>(0f32).0),
            Simd::from(rhs.embed::<4>(0f32).0),
        );
        Vec3f32::new_from_vec(&(a - b).to_array().to_vec())
    }
}

impl ops::Mul for &Vec3f32 {
    type Output = f32;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        let (a, b) = (
            Simd::from(self.embed::<4>(0f32).0),
            Simd::from(rhs.embed::<4>(0f32).0),
        );
        Simd::from(a * b).reduce_sum()
    }
}

impl ops::Mul<f32> for &Vec3f32 {
    type Output = Vec3f32;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        let (a, b) = (Simd::from(self.embed::<4>(0f32).0), Simd::splat(rhs));
        Vec3f32::new_from_vec(&(a * b).to_array().to_vec())
    }
}

impl ops::Div<f32> for &Vec3f32 {
    type Output = Vec3f32;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        let (a, b) = (Simd::from(self.embed::<4>(0f32).0), Simd::splat(rhs));
        Vec3f32::new_from_vec(&(a / b).to_array().to_vec())
    }
}

impl Vec3f32 {
    #[inline(always)]
    pub fn norm_l2(&self) -> f32 {
        (self * self).sqrt()
    }

    #[inline(always)]
    pub fn normalize(&self) -> Self {
        self / self.norm_l2()
    }

    #[inline(always)]
    pub fn cross(&self, rhs: &Self) -> Self {
        let a = self;
        let b = rhs;
        Self([
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ])
    }
}

impl<T, const N: usize> ops::Index<usize> for Vector<T, N>
where
    T: Default + Debug + Copy + SimdElement,
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<T, const N: usize> ops::IndexMut<usize> for Vector<T, N>
where
    T: Default + Debug + Copy + SimdElement,
{
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Default + Debug + Copy + SimdElement,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self([Default::default(); N])
    }

    #[inline(always)]
    pub fn new_from_vec(src: &Vec<T>) -> Self {
        let inner: [T; N] = src
            .into_iter()
            .take(N)
            .map(|&a| a)
            .collect::<Vec<T>>()
            .try_into()
            .unwrap();
        Self(inner)
    }

    #[inline(always)]
    pub fn new_from_array(src: [T; N]) -> Self {
        Self(src)
    }

    #[inline(always)]
    pub fn embed<const M: usize>(&self, value: T) -> Vector<T, M> {
        assert!(N < M);
        let mut vec: Vector<T, M> = Vector::new();
        for i in 0..M {
            vec[i] = if i < N { self[i] } else { value };
        }
        vec
    }

    #[inline(always)]
    pub fn project<const M: usize>(&self) -> Vector<T, M> {
        assert!(N > M);
        let mut vec: Vector<T, M> = Vector::new();
        for i in 0..M {
            vec[i] = self[i]
        }
        vec
    }
}
