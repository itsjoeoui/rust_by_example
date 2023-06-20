#![allow(dead_code)] // such that we can have unused vars only for now

use core::fmt;

// a unit struct
struct Unit;

// a tuple struct
struct Pair(i32, f32);

// a named tuple struct
#[derive(Clone, Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// of course it can re-use other structs
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rectangle: top_left: ({}, {}), bottom_right: ({}, {})",
            self.top_left.x, self.top_left.y, self.bottom_right.x, self.bottom_right.y
        )
    }
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;
    (x2 - x1) * (y2 - y1)
}

impl Rectangle {
    fn area(&self) -> f32 {
        let Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } = self;
        (x2 - x1) * (y2 - y1)
    }
}

fn square(pt: &Point, len: f32) -> Rectangle {
    Rectangle {
        top_left: pt.clone(),
        bottom_right: Point {
            x: pt.x + len,
            y: pt.y - len,
        },
    }
}

fn main() {
    println!("Hello, world!");
    let name = String::from("Joey");
    let age = 30;
    let person: Person = Person { name, age };
    println!("{:?}", person);

    // this is the struct update syntax
    let person: Person = Person { age: 31, ..person };
    println!("{:?}", person);

    // we can also destructre it with a let binding
    let Person {
        name: dname,
        age: dage,
    } = person;
    println!("{}{}", dname, dage);

    // we can also create a unit struct like this
    let _unit = Unit;

    let my_rect = Rectangle {
        top_left: Point { x: 2.0, y: 3.0 },
        bottom_right: Point { x: 6.0, y: 1.0 },
    };

    println!(
        "The area of my rect is {} and {}",
        rect_area(&my_rect),
        my_rect.area()
    );

    println!(
        "this is a square \n {}",
        square(&Point { x: 0.0, y: 1.0 }, 1.0)
    )
}
