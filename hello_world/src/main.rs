use std::fmt::{self, Debug};

struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(DebugPrintable);

impl fmt::Display for DebugPrintable {
    // The signature must be like this, it is required.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let list = &self.0;

        // we put a bunch of ? here is because we want to throw an error
        // right away if there is one.
        write!(f, "[")?;

        for (idx, val) in list.iter().enumerate() {
            if idx != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", idx, val)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RGB ({r}, {g}, {b}) 0x{r:0>2X}{g:0>2X}{b:0>2X}",
            r = self.r,
            g = self.g,
            b = self.b
        )
    }
}

fn main() {
    println!("Hello, world!");
    println!("{} days", 31);
    // it can take positional arguments
    println!("{1}, this is {0}. {0}, this is {1}", "Alice", "Bob");

    // dis shit is nice
    println!("{name} is {age} years old", name = "Alice", age = 30);
    // more: https://doc.rust-lang.org/stable/rust-by-example/hello/print.html
    //
    // note that only types that implement fmt::Display can be printed out with
    // {}. User defined types do not implement fmt::Display by default.
    let number: f64 = 1.0;
    let width: usize = 5;
    // from Rust 1.58 or above you can directly capture the surrounding vars
    // this will print number that will be aligned on the right, padded with "-"
    println!("{number:->width$}");

    // printing this will error out
    let _: UnPrintable = UnPrintable(99);

    println!("{:?}", DebugPrintable(12));

    // you can enable pretty printing ith :#?
    println!("{:#?}", Deep(DebugPrintable(12)));

    // while debug is useful, we cannot custimize how it prints out
    // to customize it, we can implement the fmt::Display trait
    println!("{}", DebugPrintable(123));

    // one problem with fmt::Display that it is hard to implement for generic types
    // https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display.html
    // for generic, there is no one good style that fits all

    println!(
        "{}",
        Complex {
            real: 3.3,
            imag: 7.2
        }
    );

    let my_list = List(vec![1, 2, 3]);
    println!("{}", my_list);

    for color in [
        Color {
            r: 128,
            g: 255,
            b: 90,
        },
        Color { r: 0, g: 3, b: 254 },
        Color { r: 0, g: 0, b: 0 },
    ] {
        println!("{}", color)
    }
}
