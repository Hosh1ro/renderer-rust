use std::{fmt::Debug, ops};

use super::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<T, const N_ROW: usize, const N_COL: usize>([Vector<T, N_COL>; N_ROW])
where
    T: Default + Debug + Copy;

impl<T, const N_ROW: usize, const N_COL: usize> ops::Index<usize> for Matrix<T, N_ROW, N_COL>
where
    T: Default + Debug + Copy,
{
    type Output = Vector<T, N_COL>;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<T, const N_ROW: usize, const N_COL: usize> ops::IndexMut<usize> for Matrix<T, N_ROW, N_COL>
where
    T: Default + Debug + Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

impl<T, const N_ROW: usize, const N_COL: usize> Matrix<T, N_ROW, N_COL>
where
    T: Default + Debug + Copy,
{
    pub fn new() -> Self {
        Self([Vector::<T, N_COL>::new(); N_ROW])
    }

    pub fn transpose(&self) -> Matrix<T, N_COL, N_ROW> {
        let mut output = Matrix::<T, N_COL, N_ROW>::new();
        for i in 0..N_COL {
            output[i] = self.get_col(i)
        }
        output
    }

    pub fn set_col(&mut self, j: usize, col: &Vector<T, N_ROW>) {
        for i in 0..N_ROW {
            self[i][j] = col[i]
        }
    }

    pub fn get_col(&self, j: usize) -> Vector<T, N_ROW> {
        let mut col = Vector::new();
        for i in 0..N_ROW {
            col[i] = self[i][j]
        }
        col
    }
}

impl<T, const N_ROW: usize, const N_COL: usize> ops::Add for &Matrix<T, N_ROW, N_COL>
where
    T: Default + Debug + Copy + ops::Add<Output = T>,
{
    type Output = Matrix<T, N_ROW, N_COL>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Matrix::<T, N_ROW, N_COL>::new();
        for i in 0..N_ROW {
            output[i] = &self[i] + &rhs[i];
        }
        output
    }
}

impl<T, const N_ROW: usize, const N_COL: usize> ops::Sub for &Matrix<T, N_ROW, N_COL>
where
    T: Default + Debug + Copy + ops::Sub<Output = T>,
{
    type Output = Matrix<T, N_ROW, N_COL>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut output = Matrix::<T, N_ROW, N_COL>::new();
        for i in 0..N_ROW {
            output[i] = &self[i] - &rhs[i];
        }
        output
    }
}

impl<T, const N_ROW_A: usize, const N_COL_A: usize, const N_COL_B: usize>
    ops::Mul<&Matrix<T, N_COL_A, N_COL_B>> for &Matrix<T, N_ROW_A, N_COL_A>
where
    T: Default + Debug + Copy + ops::Mul<Output = T> + ops::Add<Output = T>,
{
    type Output = Matrix<T, N_ROW_A, N_COL_B>;

    fn mul(self, rhs: &Matrix<T, N_COL_A, N_COL_B>) -> Self::Output {
        let mut output = Matrix::<T, N_ROW_A, N_COL_B>::new();
        for i in 0..N_ROW_A {
            for j in 0..N_COL_B {
                for k in 0..N_COL_A {
                    output[i][j] = output[i][j] + self[i][k] * rhs[k][j];
                }
            }
        }
        output
    }
}

impl<T, const N_ROW: usize, const N_COL: usize> ops::Mul<&Vector<T, N_COL>>
    for &Matrix<T, N_ROW, N_COL>
where
    T: Default + Debug + Copy + ops::Mul<Output = T> + ops::Add<Output = T>,
{
    type Output = Vector<T, N_ROW>;

    fn mul(self, rhs: &Vector<T, N_COL>) -> Self::Output {
        let mut output = Vector::<T, N_ROW>::new();
        for i in 0..N_ROW {
            output[i] = &self[i] * rhs;
        }
        output
    }
}

impl<T, const N_ROW: usize, const N_COL: usize> ops::Mul<T> for &Matrix<T, N_ROW, N_COL>
where
    T: Default + Debug + Copy + ops::Mul<Output = T>,
{
    type Output = Matrix<T, N_ROW, N_COL>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut output = Matrix::<T, N_ROW, N_COL>::new();
        for i in 0..N_ROW {
            output[i] = &self[i] * rhs;
        }
        output
    }
}

impl<T, const N_ROW: usize, const N_COL: usize> ops::Div<T> for &Matrix<T, N_ROW, N_COL>
where
    T: Default + Debug + Copy + ops::Div<Output = T>,
{
    type Output = Matrix<T, N_ROW, N_COL>;

    fn div(self, rhs: T) -> Self::Output {
        let mut output = Matrix::<T, N_ROW, N_COL>::new();
        for i in 0..N_ROW {
            output[i] = &self[i] / rhs;
        }
        output
    }
}

impl<T> Matrix<T, 4, 4>
where
    T: Default
        + Debug
        + Copy
        + ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
        + ops::Neg<Output = T>,
{
    pub fn minor(&self, i: usize, j: usize) -> Matrix<T, 3, 3> {
        let mut output = Matrix::<T, 3, 3>::new();
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

    pub fn cofactor(&self, i: usize, j: usize) -> T {
        let res = self.minor(i, j).det();
        if (i + j) & 1 == 0 {
            res
        } else {
            -res
        }
    }

    pub fn det(&self) -> T {
        let mut res: T = Default::default();
        for j in 0..4 {
            res = res + self[0][j] * self.cofactor(0, j);
        }
        res
    }

    pub fn adj(&self) -> Matrix<T, 4, 4> {
        let mut output = Matrix::<T, 4, 4>::new();
        for i in 0..4 {
            for j in 0..4 {
                output[i][j] = self.cofactor(i, j);
            }
        }
        output.transpose()
    }

    pub fn inv(&self) -> Matrix<T, 4, 4> {
        &self.adj() / self.det()
    }
}

impl<T> Matrix<T, 3, 3>
where
    T: Default
        + Debug
        + Copy
        + ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
        + ops::Neg<Output = T>,
{
    pub fn minor(&self, i: usize, j: usize) -> Matrix<T, 2, 2> {
        let mut output = Matrix::<T, 2, 2>::new();
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

    pub fn cofactor(&self, i: usize, j: usize) -> T {
        let res = self.minor(i, j).det();
        if (i + j) & 1 == 0 {
            res
        } else {
            -res
        }
    }

    pub fn det(&self) -> T {
        self[0][0] * self[1][1] * self[2][2]
            + self[0][1] * self[1][2] * self[2][0]
            + self[0][2] * self[1][0] * self[2][1]
            - self[0][0] * self[1][2] * self[2][1]
            - self[0][1] * self[1][0] * self[2][2]
            - self[0][2] * self[1][1] * self[2][0]
    }

    pub fn adj(&self) -> Matrix<T, 3, 3> {
        let mut output = Matrix::<T, 3, 3>::new();
        for i in 0..3 {
            for j in 0..3 {
                output[i][j] = self.cofactor(i, j);
            }
        }
        output.transpose()
    }

    pub fn inv(&self) -> Matrix<T, 3, 3> {
        &self.adj() / self.det()
    }
}

impl<T> Matrix<T, 2, 2>
where
    T: Default
        + Debug
        + Copy
        + ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
        + ops::Neg<Output = T>,
{
    pub fn minor(&self, i: usize, j: usize) -> Matrix<T, 1, 1> {
        let mut output = Matrix::<T, 1, 1>::new();
        for i_ in 0..1 {
            for j_ in 0..1 {
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

    pub fn cofactor(&self, i: usize, j: usize) -> T {
        let res = self.minor(i, j).det();
        if (i + j) & 1 == 0 {
            res
        } else {
            -res
        }
    }

    pub fn det(&self) -> T {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    pub fn adj(&self) -> Matrix<T, 2, 2> {
        let mut output = Matrix::<T, 2, 2>::new();
        for i in 0..2 {
            for j in 0..2 {
                output[i][j] = self.cofactor(i, j);
            }
        }
        output.transpose()
    }

    pub fn inv(&self) -> Matrix<T, 2, 2> {
        let mut output = Matrix::<T, 2, 2>::new();
        (output[0][0], output[0][1]) = (self[1][1], -self[0][1]);
        (output[1][0], output[1][1]) = (-self[1][0], self[0][0]);
        output
    }
}

impl<T> Matrix<T, 1, 1>
where
    T: Default + Debug + Copy + ops::Mul<Output = T> + ops::Add<Output = T> + ops::Neg,
{
    pub fn det(&self) -> T {
        self[0][0]
    }
}
