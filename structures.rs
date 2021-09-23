#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) {
    let Rectangle {
        top_left: Point {
            x: height,
            y: width,
        },
        bottom_right: _,
    } = rectangle;
    let area = height * width;
    println!("{}", area);
}

fn square(p: Point, i: f32) {
    let lower_left: Point = p;
    let width: f32 = i;
    let height: f32 = i;
    let rect = Rectangle {
        top_left: lower_left,
        bottom_right: Point {
            x: height,
            y: width,
        },
    };
    //return(&static Rectangle);
    println!("{}", rect.top_left);
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a 'Point'
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // 'bottom_right.y' will be the same as 'point.y' because we used that field
    // from 'point'
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a 'let' binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        //struct initiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Initialise a unit struct
    let _unit = Unit;

    // Initialise a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    rect_area(_rectangle);
}
