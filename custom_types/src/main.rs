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

enum FlowEvent {
    ReachedEnd,
    NodeComplete { next: String, output: String },
    NodeFailed(String),
}

// you can also do type alias
#[derive(Debug)]
enum LoggerLoggerEqualsLoggerGetLogger {
    Java,
    Insane,
}

type Dont = LoggerLoggerEqualsLoggerGetLogger;

impl LoggerLoggerEqualsLoggerGetLogger {
    fn log(&self) {
        match self {
            // Here instead of putting the super long class name
            // We can just put Self instead! Super nice
            Self::Java => println!("Java is stupid"),
            Self::Insane => println!("Insane is stupid"),
        }
    }
}

fn inspect(event: FlowEvent) {
    // Rust enum is amazing!!
    use FlowEvent::{NodeComplete, NodeFailed, ReachedEnd};
    // We can use the "use" keyword to bring things into the scope!!!

    match event {
        ReachedEnd => println!("Reached end!"),
        NodeComplete { next, output } => {
            println!("Done: {}\n Moving towards {}", next, output)
        }
        NodeFailed(reason) => println!("Node {} failed to run", reason),
    }
}

// very much like OCaml
#[derive(Debug)]
enum LinkedList {
    Nil,
    Cons(i32, Box<LinkedList>),
}

impl LinkedList {
    fn new() -> LinkedList {
        Self::Nil
    }

    // what is the difference between self and &self.
    // the self here takes the ownership of the object
    // where as &self borrows a reference to the object
    // and this reference is also an immutable reference (by default)

    fn prepend(self, elem: i32) -> LinkedList {
        Self::Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        // Note that match on the concrete type is prefered over
        // matching on the reference type
        match *self {
            LinkedList::Nil => 0,
            // note that we cannot put xs here because that will make us
            // claim the ownership of xs, instead we can use "ref xs"
            LinkedList::Cons(_, ref xs) => 1 + xs.len(),
        }
    }

    // this one will print out right away
    fn print(&self) {
        match *self {
            LinkedList::Nil => println!("Nil"),
            LinkedList::Cons(x, ref xs) => {
                println!("{}", x);
                xs.print()
            }
        }
    }

    // format! returns a heap allocated string that can be printed out later
    fn stringify(&self) -> String {
        match *self {
            LinkedList::Nil => format!("Nil"),
            LinkedList::Cons(x, ref xs) => format!("{}, {}", x, xs.stringify()),
        }
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
    );

    let failed = FlowEvent::NodeFailed(String::from("BRUHHH"));
    inspect(failed);

    let stupid = Dont::Java;
    println!("{:?}", stupid);

    println!(
        "you can cast enum to an integer {} {:?}",
        Dont::Java as i32,
        Dont::Java
    );

    use LinkedList::{Cons, Nil};

    // think of Box as a smart pointer
    // it lives on the stack and points to the heap
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{:?}", list);

    let mut test_list = LinkedList::new();
    test_list = test_list.prepend(3);
    test_list = test_list.prepend(2);
    test_list = test_list.prepend(1);
    test_list.print();

    println!("{}", test_list.stringify())
}
