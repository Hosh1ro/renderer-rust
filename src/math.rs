use std::ops;

#[derive(Debug, Clone, Copy)]
struct Vec<T, const N: usize>([T; N])
where
    T: Default + Copy;

impl<T, const N: usize> ops::Add for &Vec<T, N>
where
    T: Default + Copy + ops::Add<Output = T>,
{
    type Output = Vec<T, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Vec::<T, N>([Default::default(); N]);

        for i in 0..N {
            output.0[i] = self.0[i] + rhs.0[i];
        }

        return output;
    }
}

impl<T, const N: usize> ops::Sub for &Vec<T, N>
where
    T: Default + Copy + ops::Sub<Output = T>,
{
    type Output = Vec<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut output = Vec::<T, N>([Default::default(); N]);

        for i in 0..N {
            output.0[i] = self.0[i] - rhs.0[i];
        }

        return output;
    }
}

impl<const N: usize> ops::Mul for &Vec<f32, N> {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut output = 0f32;

        for i in 0..N {
            output += self.0[i] * rhs.0[i];
        }

        return output;
    }
}

impl<const N: usize> ops::Mul for &Vec<f64, N> {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut output = 0f64;

        for i in 0..N {
            output += self.0[i] * rhs.0[i];
        }

        return output;
    }
}

impl<T, const N: usize> ops::Mul<f32> for &Vec<T, N>
where
    T: Default + Copy + ops::Mul<f32, Output = T>,
{
    type Output = Vec<T, N>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut output = Vec::<T, N>([Default::default(); N]);

        for i in 0..N {
            output.0[i] = self.0[i] * rhs;
        }

        return output;
    }
}

impl<T, const N: usize> ops::Mul<f64> for &Vec<T, N>
where
    T: Default + Copy + ops::Mul<f64, Output = T>,
{
    type Output = Vec<T, N>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut output = Vec::<T, N>([Default::default(); N]);

        for i in 0..N {
            output.0[i] = self.0[i] * rhs;
        }

        return output;
    }
}

impl<T, const N: usize> ops::Mul<&Vec<T, N>> for f32
where
    T: Default + Copy + ops::Mul<f32, Output = T>,
{
    type Output = Vec<T, N>;

    fn mul(self, rhs: &Vec<T, N>) -> Self::Output {
        let mut output = Vec::<T, N>([Default::default(); N]);

        for i in 0..N {
            output.0[i] = rhs.0[i] * self;
        }

        return output;
    }
}

impl<T, const N: usize> ops::Mul<&Vec<T, N>> for f64
where
    T: Default + Copy + ops::Mul<f64, Output = T>,
{
    type Output = Vec<T, N>;

    fn mul(self, rhs: &Vec<T, N>) -> Self::Output {
        let mut output = Vec::<T, N>([Default::default(); N]);

        for i in 0..N {
            output.0[i] = rhs.0[i] * self;
        }

        return output;
    }
}

impl<T, const N: usize> ops::Div<f32> for &Vec<T, N>
where
    T: Default + Copy + ops::Div<f32, Output = T>,
{
    type Output = Vec<T, N>;

    fn div(self, rhs: f32) -> Self::Output {
        let mut output = Vec::<T, N>([Default::default(); N]);

        for i in 0..N {
            output.0[i] = self.0[i] / rhs;
        }

        return output;
    }
}

impl<T, const N: usize> ops::Div<f64> for &Vec<T, N>
where
    T: Default + Copy + ops::Div<f64, Output = T>,
{
    type Output = Vec<T, N>;

    fn div(self, rhs: f64) -> Self::Output {
        let mut output = Vec::<T, N>([Default::default(); N]);

        for i in 0..N {
            output.0[i] = self.0[i] / rhs;
        }

        return output;
    }
}

impl<const N: usize> Vec<f32, N> {
    fn norm_l2(&self) -> f32 {
        (self * self).sqrt()
    }

    fn nomalize(&self) -> Self {
        self / self.norm_l2()
    }
}

impl<const N: usize> Vec<f64, N> {
    fn norm_l2(&self) -> f64 {
        (self * self).sqrt()
    }

    fn nomalize(&self) -> Self {
        self / self.norm_l2()
    }
}

impl Vec<f32, 3> {
    fn cross(&self, rhs: &Self) -> Self {
        let a = self.0;
        let b = rhs.0;
        Self([
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ])
    }
}

impl Vec<f64, 3> {
    fn cross(&self, rhs: &Self) -> Self {
        let a = self.0;
        let b = rhs.0;
        Self([
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ])
    }
}

type Vec2f = Vec<f32, 2>;
type Vec2d = Vec<f64, 2>;
type Vec3f = Vec<f32, 3>;
type Vec3d = Vec<f64, 3>;

struct Mat<T, const N_ROW: usize, const N_COL: usize>
where
    T: Default + Copy,
{
    mat: [Vec<T, N_COL>; N_ROW],
}

impl<T, const N_ROW: usize, const N_COL: usize> ops::Add for &Mat<T, N_ROW, N_COL>
where
    T: Default + Copy + ops::Add<Output = T>,
{
    type Output = Mat<T, N_ROW, N_COL>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Mat::<T, N_ROW, N_COL> {
            mat: [Vec::<T, N_COL>([Default::default(); N_COL]); N_ROW],
        };

        for i in 0..N_ROW {
            output.mat[i] = &self.mat[i] + &rhs.mat[i];
        }

        return output;
    }
}

impl<T, const N_ROW: usize, const N_COL: usize> ops::Sub for &Mat<T, N_ROW, N_COL>
where
    T: Default + Copy + ops::Sub<Output = T>,
{
    type Output = Mat<T, N_ROW, N_COL>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut output = Mat::<T, N_ROW, N_COL> {
            mat: [Vec::<T, N_COL>([Default::default(); N_COL]); N_ROW],
        };

        for i in 0..N_ROW {
            output.mat[i] = &self.mat[i] - &rhs.mat[i];
        }

        return output;
    }
}
