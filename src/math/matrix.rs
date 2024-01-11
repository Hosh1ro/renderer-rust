use std::{fmt::Debug, ops, simd::SimdElement};

use super::vector::{Vec2f32, Vec3f32, Vec4f32, Vector};

#[derive(Debug, Clone, Copy)]
pub struct Matrix<T, const N_ROW: usize, const N_COL: usize>([Vector<T, N_COL>; N_ROW])
where
    T: Default + Debug + Copy + SimdElement;

type Matrix2f32 = Matrix<f32, 2, 2>;
type Matrix2f64 = Matrix<f64, 2, 2>;
type Matrix3f32 = Matrix<f32, 3, 3>;
type Matrix3f64 = Matrix<f64, 3, 3>;
type Matrix4f32 = Matrix<f32, 4, 4>;
type Matrix4f64 = Matrix<f64, 4, 4>;

impl ops::Add for &Matrix4f32 {
    type Output = Matrix4f32;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Matrix4f32::new();

        output[0] = &self[0] + &rhs[0];
        output[1] = &self[1] + &rhs[1];
        output[2] = &self[2] + &rhs[2];
        output[3] = &self[3] + &rhs[3];

        output
    }
}

impl ops::Sub for &Matrix4f32 {
    type Output = Matrix4f32;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut output = Matrix4f32::new();

        output[0] = &self[0] - &rhs[0];
        output[1] = &self[1] - &rhs[1];
        output[2] = &self[2] - &rhs[2];
        output[3] = &self[3] - &rhs[3];

        output
    }
}

impl ops::Mul for &Matrix4f32 {
    type Output = Matrix4f32;

    #[inline(always)]
    fn mul(self, rhs: &Matrix4f32) -> Self::Output {
        let mut output = Matrix4f32::new();
        let b_col = [
            rhs.get_col(0),
            rhs.get_col(1),
            rhs.get_col(2),
            rhs.get_col(3),
        ];

        for (i, row) in self.0.iter().enumerate() {
            output[i][0] = row * &b_col[0];
            output[i][1] = row * &b_col[1];
            output[i][2] = row * &b_col[2];
            output[i][3] = row * &b_col[3];
        }

        output
    }
}

impl ops::Mul<&Vec4f32> for &Matrix4f32 {
    type Output = Vec4f32;

    #[inline(always)]
    fn mul(self, rhs: &Vec4f32) -> Self::Output {
        let mut output = Vec4f32::new();

        output[0] = &self[0] * rhs;
        output[1] = &self[1] * rhs;
        output[2] = &self[2] * rhs;
        output[3] = &self[3] * rhs;

        output
    }
}

impl ops::Mul<f32> for &Matrix4f32 {
    type Output = Matrix4f32;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        let mut output = Matrix4f32::new();

        output[0] = &self[0] * rhs;
        output[1] = &self[1] * rhs;
        output[2] = &self[2] * rhs;
        output[3] = &self[3] * rhs;

        output
    }
}

impl ops::Div<f32> for &Matrix4f32 {
    type Output = Matrix4f32;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        let mut output = Matrix4f32::new();

        output[0] = &self[0] / rhs;
        output[1] = &self[1] / rhs;
        output[2] = &self[2] / rhs;
        output[3] = &self[3] / rhs;

        output
    }
}

impl Matrix4f32 {
    #[inline(always)]
    pub fn minor(&self, i: usize, j: usize) -> Matrix3f32 {
        let mut output = Matrix3f32::new();
        for i_ in 0..3 {
            for j_ in 0..3 {
                output[i_][j_] = if i_ < i {
                    if j_ < j {
                        self[i_][j_]
                    } else {
                        self[i_][j_ + 1]
                    }
                } else {
                    if j_ < j {
                        self[i_ + 1][j_]
                    } else {
                        self[i_ + 1][j_ + 1]
                    }
                }
            }
        }
        output
    }

    #[inline(always)]
    pub fn cofactor(&self, i: usize, j: usize) -> f32 {
        let res = self.minor(i, j).det();
        if (i + j) & 1 == 0 {
            res
        } else {
            -res
        }
    }

    #[inline(always)]
    pub fn det(&self) -> f32 {
        let mut res = Default::default();
        for j in 0..4 {
            res = res + self[0][j] * self.cofactor(0, j);
        }
        res
    }

    #[inline(always)]
    pub fn adj(&self) -> Matrix4f32 {
        let mut output = Matrix4f32::new();
        for i in 0..4 {
            for j in 0..4 {
                output[i][j] = self.cofactor(i, j);
            }
        }
        output.transpose()
    }

    #[inline(always)]
    pub fn inv(&self) -> Matrix4f32 {
        &self.adj() / self.det()
    }
}

impl ops::Add for &Matrix2f32 {
    type Output = Matrix2f32;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Matrix2f32::new();

        output[0] = &self[0] + &rhs[0];
        output[1] = &self[1] + &rhs[1];

        output
    }
}

impl ops::Sub for &Matrix2f32 {
    type Output = Matrix2f32;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut output = Matrix2f32::new();

        output[0] = &self[0] - &rhs[0];
        output[1] = &self[1] - &rhs[1];

        output
    }
}

impl ops::Mul for &Matrix2f32 {
    type Output = Matrix2f32;

    #[inline(always)]
    fn mul(self, rhs: &Matrix2f32) -> Self::Output {
        let mut output = Matrix2f32::new();
        let b_col = [rhs.get_col(0), rhs.get_col(1)];

        for (i, row) in self.0.iter().enumerate() {
            output[i][0] = row * &b_col[0];
            output[i][1] = row * &b_col[1];
        }

        output
    }
}

impl ops::Mul<&Vec2f32> for &Matrix2f32 {
    type Output = Vec2f32;

    #[inline(always)]
    fn mul(self, rhs: &Vec2f32) -> Self::Output {
        let mut output = Vec2f32::new();

        output[0] = &self[0] * rhs;
        output[1] = &self[1] * rhs;

        output
    }
}

impl ops::Mul<f32> for &Matrix2f32 {
    type Output = Matrix2f32;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        let mut output = Matrix2f32::new();

        output[0] = &self[0] * rhs;
        output[1] = &self[1] * rhs;

        output
    }
}

impl ops::Div<f32> for &Matrix2f32 {
    type Output = Matrix2f32;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        let mut output = Matrix2f32::new();

        output[0] = &self[0] / rhs;
        output[1] = &self[1] / rhs;

        output
    }
}

impl Matrix2f32 {
    #[inline(always)]
    pub fn det(&self) -> f32 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    #[inline(always)]
    pub fn inv(&self) -> Matrix2f32 {
        let mut output = Matrix2f32::new();
        (output[0][0], output[0][1]) = (self[1][1], -self[0][1]);
        (output[1][0], output[1][1]) = (-self[1][0], self[0][0]);
        output
    }
}

impl ops::Add for &Matrix3f32 {
    type Output = Matrix3f32;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Matrix3f32::new();

        output[0] = &self[0] + &rhs[0];
        output[1] = &self[1] + &rhs[1];
        output[2] = &self[1] + &rhs[2];

        output
    }
}

impl ops::Sub for &Matrix3f32 {
    type Output = Matrix3f32;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut output = Matrix3f32::new();

        output[0] = &self[0] - &rhs[0];
        output[1] = &self[1] - &rhs[1];
        output[2] = &self[1] - &rhs[2];

        output
    }
}

impl ops::Mul for &Matrix3f32 {
    type Output = Matrix3f32;

    #[inline(always)]
    fn mul(self, rhs: &Matrix3f32) -> Self::Output {
        let mut output = Matrix3f32::new();
        let b_col = [rhs.get_col(0), rhs.get_col(1), rhs.get_col(2)];

        for (i, row) in self.0.iter().enumerate() {
            output[i][0] = row * &b_col[0];
            output[i][1] = row * &b_col[1];
            output[i][2] = row * &b_col[2];
        }

        output
    }
}

impl ops::Mul<&Vec3f32> for &Matrix3f32 {
    type Output = Vec3f32;

    #[inline(always)]
    fn mul(self, rhs: &Vec3f32) -> Self::Output {
        let mut output = Vec3f32::new();

        output[0] = &self[0] * rhs;
        output[1] = &self[1] * rhs;
        output[2] = &self[2] * rhs;

        output
    }
}

impl ops::Mul<f32> for &Matrix3f32 {
    type Output = Matrix3f32;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        let mut output = Matrix3f32::new();

        output[0] = &self[0] * rhs;
        output[1] = &self[1] * rhs;
        output[2] = &self[2] * rhs;

        output
    }
}

impl ops::Div<f32> for &Matrix3f32 {
    type Output = Matrix3f32;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        let mut output = Matrix3f32::new();

        output[0] = &self[0] / rhs;
        output[1] = &self[1] / rhs;
        output[2] = &self[2] / rhs;

        output
    }
}

impl Matrix3f32 {
    #[inline(always)]
    pub fn minor(&self, i: usize, j: usize) -> Matrix2f32 {
        let mut output = Matrix2f32::new();
        for i_ in 0..2 {
            for j_ in 0..2 {
                output[i_][j_] = if i_ < i {
                    if j_ < j {
                        self[i_][j_]
                    } else {
                        self[i_][j_ + 1]
                    }
                } else {
                    if j_ < j {
                        self[i_ + 1][j_]
                    } else {
                        self[i_ + 1][j_ + 1]
                    }
                }
            }
        }
        output
    }

    #[inline(always)]
    pub fn cofactor(&self, i: usize, j: usize) -> f32 {
        let res = self.minor(i, j).det();
        if (i + j) & 1 == 0 {
            res
        } else {
            -res
        }
    }

    #[inline(always)]
    pub fn det(&self) -> f32 {
        self[0][0] * self[1][1] * self[2][2]
            + self[0][1] * self[1][2] * self[2][0]
            + self[0][2] * self[1][0] * self[2][1]
            - self[0][0] * self[1][2] * self[2][1]
            - self[0][1] * self[1][0] * self[2][2]
            - self[0][2] * self[1][1] * self[2][0]
    }

    #[inline(always)]
    pub fn adj(&self) -> Matrix3f32 {
        let mut output = Matrix3f32::new();
        for i in 0..3 {
            for j in 0..3 {
                output[i][j] = self.cofactor(i, j);
            }
        }
        output.transpose()
    }

    #[inline(always)]
    pub fn inv(&self) -> Matrix3f32 {
        &self.adj() / self.det()
    }
}

impl<T> Matrix<T, 1, 1>
where
    T: Default
        + Debug
        + Copy
        + SimdElement
        + ops::Mul<Output = T>
        + ops::Add<Output = T>
        + ops::Neg,
{
    #[inline(always)]
    pub fn det(&self) -> T {
        self[0][0]
    }
}

impl<T, const N_ROW: usize, const N_COL: usize> ops::Index<usize> for Matrix<T, N_ROW, N_COL>
where
    T: Default + Debug + Copy + SimdElement,
{
    type Output = Vector<T, N_COL>;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<T, const N_ROW: usize, const N_COL: usize> ops::IndexMut<usize> for Matrix<T, N_ROW, N_COL>
where
    T: Default + Debug + Copy + SimdElement,
{
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

impl<T, const N_ROW: usize, const N_COL: usize> Matrix<T, N_ROW, N_COL>
where
    T: Default + Debug + Copy + SimdElement,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self([Vector::<T, N_COL>::new(); N_ROW])
    }

    #[inline(always)]
    pub fn transpose(&self) -> Matrix<T, N_COL, N_ROW> {
        let mut output = Matrix::<T, N_COL, N_ROW>::new();
        for i in 0..N_COL {
            output[i] = self.get_col(i)
        }
        output
    }

    #[inline(always)]
    pub fn set_col(&mut self, j: usize, col: &Vector<T, N_ROW>) {
        for i in 0..N_ROW {
            self[i][j] = col[i]
        }
    }

    #[inline(always)]
    pub fn get_col(&self, j: usize) -> Vector<T, N_ROW> {
        let mut col = Vector::new();
        for i in 0..N_ROW {
            col[i] = self[i][j]
        }
        col
    }
}
