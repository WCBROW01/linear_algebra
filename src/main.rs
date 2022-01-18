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

	foo /= 2;
	println!("Dividing a matrix by a scalar:");
	foo.print();
}
