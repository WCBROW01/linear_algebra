mod matrix;
use matrix::Matrix;

fn main() {
	let mut test = Matrix::<i32, 2, 3>::new();
	println!("Rows: {}, Columns: {}", test.rows(), test.cols());
	println!("Zero-initialized:");
	test.print();

	test[0][1] = 1;
	println!("Assigning a value within the matrix:");
	test.print();

	let mut foo = Matrix([[2, 4, 5],
					  [6, 7, 8]]);
	println!("Instantiating a pre-made matrix:");
	foo.print();

	println!("\nArithmetic:");
	test += foo;
	println!("Adding to the matrix:");
	test.print();

	test -= foo;
	println!("Subtracting from the matrix:");
	test.print();

	let bar = -foo;
	println!("Negating a matrix:");
	bar.print();

	foo *= 2;
	println!("Multiplying a matrix by a scalar:");
	foo.print();

	println!("\nElementary row operations:");
	foo.swap_rows(0, 1);
	println!("Swapping row 0 and 1 of a matrix:");
	foo.print();

	foo.mul_row_by_scalar(0, 2);
	println!("Multiplying a row by a scalar:");
	foo.print();

	foo.add_row_to_other(0, 1, 1);
	println!("Adding a multiple of a row to another row:");
	foo.print();

	let baz = Matrix([[1, 2, 3],
					  [4, 5, 6]]);

	let qux = Matrix([[ 7,  8],
					  [ 9, 10],
					  [11, 12]]);

	let quux = baz * qux;
	println!("Multiplying a matrix by another matrix:");
	quux.print();

	let rr: Matrix<f64, 3, 3> = Matrix([
		[1.0, 3.0, 6.0],
		[2.0, -9.0, 4.0],
		[11.0, 13.0, 28.0]
	]);
	println!("Creating a new matrix for row reduction:");
	rr.print();
	let rr2 = rr.row_reduce();
	println!("Row-reducing the matrix:");
	rr2.print();
}
