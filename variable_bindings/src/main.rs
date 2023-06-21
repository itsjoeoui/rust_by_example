fn main() {
    // put an _ before can supress un-used warnings
    let _x = 1;
    let mut y = _x;
    y += 1;
    println!("{} {}", _x, y);
    println!("Hello, world!");

    // in rust you can shadle bind a variable just like in OCaml
    let a = 1;
    {
        let a = 2;
        println!("a = {}", a);
    }
    println!("a = {}", a);

    // note that we can also initialize a variable and then bind to it later
    // though notice that if a variable gets used before it gets intialized,
    // the compiler will block that behavior and it will not compile
    // this way, we are sure we won't have an un-initialized variable which
    // leads to undefined behavior in languages like C and C++

    // another idea is that we can freeze a variable
    // for example

    let mut _this_is_mutable = 1;
    {
        let _this_is_mutable = 2;

        // this line below will error out since the variable is now frozen
        // _this_is_mutable = 3;

        println!("this is mutable = {}", _this_is_mutable);
    }
    println!("this is mutable = {}", _this_is_mutable);
    //
}
