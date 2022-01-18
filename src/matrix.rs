use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Neg};
use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct Matrix<T: Default + Copy, const M: usize, const N: usize>(pub [[T; N]; M]);

impl<T: Default + Copy, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn rows(&self) -> usize {
		M
	}

	pub fn cols(&self) -> usize {
		N
	}

	pub fn new() -> Self {
		Matrix([[Default::default(); N]; M])
	}
}

impl<T: Default + Copy + Display, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn print(&self) {
		for i in 0..M {
			for j in 0..N {
				print!("{} ", self[i][j]);
			}
			println!();
		}
	}
}

impl<T: Default + Copy, const M: usize, const N: usize> Default for Matrix<T, M, N> {
	fn default() -> Self {
		Matrix([[Default::default(); N]; M])
	}
}

impl<T: Default + Copy, const M: usize, const N: usize> Index<usize> for Matrix<T, M, N> {
	type Output = [T; N];

	fn index(&self, i: usize) -> &Self::Output {
		&self.0[i]
	}
}


impl<T: Default + Copy, const M: usize, const N: usize> IndexMut<usize> for Matrix<T, M, N> {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.0[i]
	}
}

impl<T: Default + Copy + Add<Output = T>, const M: usize, const N: usize> Add for Matrix<T, M, N> {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		let mut new_matrix = self;
		for i in 0..M {
			for j in 0..N {
				new_matrix[i][j] = self[i][j] + other[i][j];
			}
		}

		new_matrix
	}
}

 
impl<T: Default + Copy + AddAssign, const M: usize, const N: usize> AddAssign for Matrix<T, M, N> {
	fn add_assign(&mut self, other: Self) {
		for i in 0..M {
			for j in 0..N {
				self[i][j] += other[i][j];
			}
		}
	}
}

impl<T: Default + Copy + Sub<Output = T>, const M: usize, const N: usize> Sub for Matrix<T, M, N> {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		let mut new_matrix = self;
		for i in 0..M {
			for j in 0..N {
				new_matrix[i][j] = self[i][j] - other[i][j];
			}
		}

		new_matrix
	}
}

 
impl<T: Default + Copy + SubAssign, const M: usize, const N: usize> SubAssign for Matrix<T, M, N> {
	fn sub_assign(&mut self, other: Self) {
		for i in 0..M {
			for j in 0..N {
				self[i][j] -= other[i][j];
			}
		}
	}
}

impl<T: Default + Copy + Neg<Output = T>, const M: usize, const N: usize> Neg for Matrix<T, M, N> {
	type Output = Self;

	fn neg(self) -> Self::Output {
		let mut new_matrix = self;
		for i in 0..M {
			for j in 0..N {
				new_matrix[i][j] = -self[i][j];
			}
		}

		new_matrix
	}
}
