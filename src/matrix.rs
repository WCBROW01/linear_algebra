use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign};
use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct Matrix<T, const M: usize, const N: usize>(pub [[T; N]; M]);

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn rows(&self) -> usize {
		M
	}

	pub fn cols(&self) -> usize {
		N
	}
}

impl<T: Copy, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn swap_rows(&mut self, a: usize, b: usize) {
		let temp = self[a];
		self[a] = self[b];
		self[b] = temp;
	}
}

impl<T: Default + Copy, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn new() -> Self {
		Matrix([[Default::default(); N]; M])
	}
}

impl<T: Copy + MulAssign, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn mul_row_by_scalar(&mut self, row: usize, scalar: T) {
		for i in 0..N {
			self[row][i] *= scalar;
		}
	}
}

impl<T: Copy + AddAssign + Mul<Output = T>, const M: usize, const N: usize> Matrix<T, M, N> {
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

impl<const M: usize, const N: usize> Matrix<f64, M, N> {
	fn reduce(&mut self, row: usize, col: usize) {
		let mut pivot = || {
			// Find pivot
			for i in row..M {
				if self[i][col] != 0.0 {
					self.swap_rows(row, i);
					return true;
				}
			}
			false
		};


		if pivot() {
			// Make zeroes below
			for i in row+1..M {
				if self[i][col] != 0.0 {
					self.add_row_to_other(row, i, -(self[i][col] / self[row][col]));
				}
			}

			if row + 1 < M && col + 1 < N {
				self.reduce(row + 1, col + 1);
			}

			// Make zeros above
			for i in 0..row {
				self.add_row_to_other(row, i, -(self[i][col] / self[row][col]));
			}

			// Make 1
			self.mul_row_by_scalar(row, 1.0 / self[row][col]);
		} else if col + 1 < N {
			self.reduce(row, col + 1);
		}
	}

	fn remove_negative_zero(&mut self) {
		for i in 0..M {
			for j in 0..N {
				if self[i][j] == -0.0 {
					self[i][j] = 0.0;
				}
			}
		}
	}

	pub fn row_reduce(mut self) -> Self {
		self.reduce(0, 0);
		self.remove_negative_zero();
		return self;
	}
}

impl<T: Display, const M: usize, const N: usize> Matrix<T, M, N> {
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

impl<T, const M: usize, const N: usize> Index<usize> for Matrix<T, M, N> {
	type Output = [T; N];

	fn index(&self, i: usize) -> &Self::Output {
		&self.0[i]
	}
}

impl<T, const M: usize, const N: usize> IndexMut<usize> for Matrix<T, M, N> {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.0[i]
	}
}

impl<T: Copy + Add<Output = T>, const M: usize, const N: usize> Add for Matrix<T, M, N> {
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


impl<T: Copy + AddAssign, const M: usize, const N: usize> AddAssign for Matrix<T, M, N> {
	fn add_assign(&mut self, other: Self) {
		for i in 0..M {
			for j in 0..N {
				self[i][j] += other[i][j];
			}
		}
	}
}

impl<T: Copy + Sub<Output = T>, const M: usize, const N: usize> Sub for Matrix<T, M, N> {
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


impl<T: Copy + SubAssign, const M: usize, const N: usize> SubAssign for Matrix<T, M, N> {
	fn sub_assign(&mut self, other: Self) {
		for i in 0..M {
			for j in 0..N {
				self[i][j] -= other[i][j];
			}
		}
	}
}

impl<T: Copy + Neg<Output = T>, const M: usize, const N: usize> Neg for Matrix<T, M, N> {
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

impl<T: Copy + Mul<Output = T>, const M: usize, const N: usize> Mul<T> for Matrix<T, M, N> {
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

impl<T: Copy + MulAssign, const M: usize, const N: usize> MulAssign<T> for Matrix<T, M, N> {
	fn mul_assign(&mut self, other: T) {
		for i in 0..M {
			for j in 0..N {
				self[i][j] *= other;
			}
		}
	}
}

impl<T: Default + Copy + Mul + AddAssign<<T as Mul>::Output>, const M: usize> MulAssign for Matrix<T, M, M> {
	fn mul_assign(&mut self, other: Matrix<T, M, M>) {
		let mut new_matrix = Matrix::new();
		for i in 0..M {
			for j in 0..M {
				for k in 0..M {
					new_matrix[i][k] += self[i][j] * other[j][k];
				}
			}
		}

		*self = new_matrix;
	}
}
