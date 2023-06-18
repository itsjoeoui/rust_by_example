fn main() {
    println!("Hello, world!");

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 + 2 = {}", 1 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    println!("1 - 2 = {}", 1 - 2);

    // it also supports the e notation
    println!("1e4 is {}", 1e4);
    // _ is also very helpful
    println!("one million is {}", 1_000_000u32)
}
