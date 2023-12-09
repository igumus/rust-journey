/**
 * Contains examples for structs and tuples
 *
 * - A Tuple is an immutable fixed-size collection of values.
 */
#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

pub fn test_struct() {
    let p0 = Point { x: 0, y: 0 };
    let mut p1 = Point { x: 12, y: 12 };

    // p0.x += 1; // it is wrong because p0 declared as immutable
    println!("zero point: {:?}", p0);
    p1.x += 1; // it is fine because p1 is declared as mutable
    println!("other point: {:?}", p1);

    // pattern matching with struct destruction.
    match p1 {
        Point { x: 13, y: yy } => println!("{}", yy),
        Point { x: 0, y: 0 } => println!("it is zero point"),
        Point { x: xx, y: yy } => println!("{}{}", xx, yy),
    }

    match p1 {
        Point { x: xx, .. } => println!("{}", xx),
    }
}

pub fn test_tuple() {
    let tuple0 = ();
    println!("Empty tuple : {:?}", tuple0);

    let tuple1: (i32, i32, f64) = (10, 20, 30.0);
    println!("Tuple1 with (i32, i32, f64): {:?}", tuple1);
    match tuple1 {
        (a, b, c) => println!("Sum of all values in Tuple1: {}", a + b + (c as i32)),
    }

    let tuple2 = (5i32, 6i32);
    println!("Tuple2 with (i32, i32): {:?}", tuple2);
    let (first, _) = tuple2;
    println!("Tuple2 first value: {}", first);
}

enum Color {
    Yellow,
    Blue,
    Green,
}

fn test_enum_color(c: Color) {
    match c {
        Color::Blue => println!("color == blue"),
        Color::Green | Color::Yellow => println!("color != blue"),
    }
}
enum Direction {
    Up,
    Down,
}

fn test_enum_direction(d: Direction) {
    match d {
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
    }
}

enum Volume {
    Off,
    Open(u8),
}
fn test_enum_volume(v: Volume) {
    match v {
        Volume::Off => println!("can not play song, volume is muted"),
        Volume::Open(0) => println!("can not play song, volume is muted"),
        Volume::Open(x) => println!("song is going to play with volume:{}", x),
    }
}

pub fn test_enums() {
    test_enum_color(Color::Blue);
    test_enum_color(Color::Green);
    test_enum_color(Color::Yellow);
    test_enum_direction(Direction::Down);
    test_enum_direction(Direction::Up);
    test_enum_volume(Volume::Off);
    test_enum_volume(Volume::Open(0));
    test_enum_volume(Volume::Open(10));
}
