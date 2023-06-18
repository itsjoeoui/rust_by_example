use core::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(mat: Matrix) -> Matrix {
    Matrix(mat.0, mat.2, mat.1, mat.3)
}

fn print_slice(slice: &[i32]) {
    for i in slice {
        println!("{}", i);
    }
}

fn main() {
    println!("Hello, world!");

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 + 2 = {}", 1 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    println!("1 - 2 = {}", 1 - 2);

    // it also supports the e notation
    println!("1e4 is {}", 1e4);
    // _ is also very helpful
    println!("one million is {}", 1_000_000u32);

    let my_pair = (1, true);
    println!("{:?}", reverse(my_pair));

    // we can easily extract values from a tuple
    println!("{}", my_pair.0);

    // ofc we can have nested tuples
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // you can also create an one element tuple, but you have to use a comma
    // for example (12,) is a tuple with one element
    //

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{}", transpose(matrix));

    // array's length is known at compile time
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", xs);
    println!("The length of this array is {}", xs.len());

    let ys: [i32; 500] = [0; 500];
    // this will give us 0 but 500 times
    println!("The length is {}", ys.len());
    println!("it occupies {} bytes", std::mem::size_of_val(&ys));
    // arrays are stored in the stack

    // a slice dynamically sized, it allows you to reference a portion of an
    // array or another slice without copying the data.
    // They are used when you want to operate on a portion of a collection such as
    // array or pass a subset of data to a function, for example
    print_slice(&xs[1..4]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(val) => println!("The value is {}", val),
            None => println!("Bro is too far with index {}", i),
        }
    }
}
