use std::{fmt::Debug, ops};

#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const N: usize>([T; N])
where
    T: Default + Debug + Copy;

impl<T, const N: usize> ops::Index<usize> for Vector<T, N>
where
    T: Default + Debug + Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<T, const N: usize> ops::IndexMut<usize> for Vector<T, N>
where
    T: Default + Debug + Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Default + Debug + Copy,
{
    pub fn new() -> Self {
        Self([Default::default(); N])
    }

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

    pub fn embed<const M: usize>(&self, value: T) -> Vector<T, M> {
        assert!(N < M);
        let mut vec: Vector<T, M> = Vector::new();
        for i in 0..M {
            vec[i] = if i < N { self[i] } else { value };
        }
        vec
    }

    pub fn project<const M: usize>(&self) -> Vector<T, M> {
        assert!(N > M);
        let mut vec: Vector<T, M> = Vector::new();
        for i in 0..M {
            vec[i] = self[i]
        }
        vec
    }
}

impl<T, const N: usize> ops::Add for &Vector<T, N>
where
    T: Default + Debug + Copy + ops::Add<Output = T>,
{
    type Output = Vector<T, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Vector::<T, N>::new();
        for i in 0..N {
            output[i] = self[i] + rhs[i];
        }
        output
    }
}

impl<T, const N: usize> ops::Sub for &Vector<T, N>
where
    T: Default + Debug + Copy + ops::Sub<Output = T>,
{
    type Output = Vector<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut output = Vector::<T, N>::new();
        for i in 0..N {
            output[i] = self[i] - rhs[i];
        }
        output
    }
}

impl<T, const N: usize> ops::Mul for &Vector<T, N>
where
    T: Default + Debug + Copy + ops::Mul<Output = T> + ops::Add<Output = T>,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut output = Default::default();
        for i in 0..N {
            output = output + self[i] * rhs[i];
        }
        output
    }
}

impl<T, const N: usize> ops::Mul<T> for &Vector<T, N>
where
    T: Default + Debug + Copy + ops::Mul<Output = T>,
{
    type Output = Vector<T, N>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut output = Vector::<T, N>::new();
        for i in 0..N {
            output[i] = self[i] * rhs;
        }
        output
    }
}

impl<T, const N: usize> ops::Div<T> for &Vector<T, N>
where
    T: Default + Debug + Copy + ops::Div<Output = T>,
{
    type Output = Vector<T, N>;

    fn div(self, rhs: T) -> Self::Output {
        let mut output = Vector::<T, N>::new();
        for i in 0..N {
            output[i] = self[i] / rhs;
        }
        output
    }
}

impl<const N: usize> Vector<f32, N> {
    pub fn norm_l2(&self) -> f32 {
        (self * self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        self / self.norm_l2()
    }
}

impl<const N: usize> Vector<f64, N> {
    pub fn norm_l2(&self) -> f64 {
        (self * self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        self / self.norm_l2()
    }
}

pub type Vec2f32 = Vector<f32, 2>;
pub type Vec2f64 = Vector<f64, 2>;
pub type Vec3f32 = Vector<f32, 3>;
pub type Vec3f64 = Vector<f64, 3>;
pub type Vec4f32 = Vector<f32, 4>;
pub type Vec4f64 = Vector<f64, 4>;

impl Vector<f32, 3> {
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

impl Vector<f64, 3> {
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
