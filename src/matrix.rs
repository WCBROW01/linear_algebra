use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign};
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

	pub fn swap_rows(&mut self, a: usize, b: usize) {
		let temp = self[a];
		self[a] = self[b];
		self[b] = temp;
	}
}

impl<T: Default + Copy + MulAssign, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn mul_row_by_scalar(&mut self, row: usize, scalar: T) {
		for i in 0..N {
			self[row][i] *= scalar;
		}
	}
}

impl<T: Default + Copy + AddAssign + Mul<Output = T>, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn add_row_to_other(&mut self, row: usize, other: usize, scalar: T) {
		/* The borrow checker will notice that &self is being borrowed twice and
		 * complain even though this operation is safe, so a pointer is created
		 * and the unsafe hatch is used to avoid making a copy of the matrix. */
		let selfptr = self as *mut Self;
		for i in 0..N {
			unsafe {
				(*selfptr)[other][i] += (*selfptr)[row][i] * scalar;
			}
		}
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

impl<T: Default + Copy + Mul<Output = T>, const M: usize, const N: usize> Mul<T> for Matrix<T, M, N> {
	type Output = Self;

	fn mul(self, other: T) -> Self::Output {
		let mut new_matrix = self;
		for i in 0..M {
			for j in 0..N {
				new_matrix[i][j] = self[i][j] * other;
			}
		}

		new_matrix
	}
}

impl<T: Default + Copy + Mul<Output = T> + AddAssign, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, N, P>> for Matrix<T, M, N> {
	type Output = Matrix<T, M, P>;

	fn mul(self, other: Matrix<T, N, P>) -> Self::Output {
		let mut new_matrix = Matrix::new();
		for i in 0..M {
			for j in 0..N {
				for k in 0..P {
					new_matrix[i][k] += self[i][j] * other[j][k];
				}
			}
		}

		new_matrix
	}
}

impl<T: Default + Copy + MulAssign, const M: usize, const N: usize> MulAssign<T> for Matrix<T, M, N> {
	fn mul_assign(&mut self, other: T) {
		for i in 0..M {
			for j in 0..N {
				self[i][j] *= other;
			}
		}
	}
}

impl<T: Default + Copy + MulAssign + AddAssign, const M: usize> MulAssign for Matrix<T, M, M> {
	type Output = Matrix<T, M, P>;

	fn mul(self, other: Matrix<T, N, P>) -> Self::Output {
		let mut new_matrix = Matrix::new();
		for i in 0..M {
			for j in 0..N {
				for k in 0..P {
					new_matrix[i][k] += self[i][j] * other[j][k];
				}
			}
		}

		new_matrix
	}
}
