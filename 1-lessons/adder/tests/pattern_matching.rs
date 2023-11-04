#[test]
fn testing_match() {
    let x = 4;
    let y = false;

    assert!(match x {
        4 => y,
        _ => !y,
    });
}

#[test]
fn test_iflet() {
    let x = 4;
    let y = false;

    assert!(if let 4 = x { y } else { !y });
}

#[test]
fn test_while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // while Some(top) = stack.pop() {
    //     println!("{}", top);
    // }
}

#[test]
fn test_let() {
    let x = 5;
    let (x, y, z) = (1, 2, 3);
    assert_eq!(x, 1);
    assert_eq!(y, 2);
    assert_eq!(z, 3);
}

#[test]
fn test_function_signature() {
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

#[test]
fn test_matching_literals() {
    let x = 1;

    assert!(match x {
        1 => true,
        _ => false,
    });
}

#[test]
fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    assert!(match x {
        Some(50) => false,
        Some(y) => y == 5,
        _ => false,
    });
}

#[test]
fn multiple_parameters() {
    let x = 1;
    let y = 2;

    match x {
        1 | 2 => assert!(true),
        _ => assert!(false),
    }

    // assert!(match (x, y) {
    //     (1, 2) => true,
    //     _ => false,
    // });
}

#[test]
fn matching_ranges() {
    let x = 5;

    assert!(match x {
        1..=5 => true,
        _ => false,
    });
}

#[test]
fn destructuring_to_Break_apart_values() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => assert_eq!(0, x),
        Point { x: 0, y } => assert_eq!(7, y),
        Point { x, y } => assert_eq!(0, x),
    }
}
#[test]
fn destructuring_enums() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => assert!(false),
        Message::Move { x, y } => assert!(false),
        Message::Write(text) => assert!(false),
        Message::ChangeColor(r, g, b) => {
            assert_eq!(0, r);
            assert_eq!(160, g);
            assert_eq!(255, b);
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

#[test]
fn destructuring_nested_enums() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => assert!(false),
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            assert_eq!(0, h);
            assert_eq!(160, s);
            assert_eq!(255, v);
        }
        _ => assert!(false),
    }
}

#[test]
fn destructuring_structs_tuples() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    assert_eq!(3, feet);
    assert_eq!(10, inches);
    assert_eq!(3, x);
    assert_eq!(-10, y);
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

#[test]
fn ignoring_values_pattern() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => assert!(false),
        _ => assert!(true),
    }

    setting_value = new_setting_value;

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => assert!(true),
        _ => assert!(false),
    }

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

#[test]
fn unused_variable() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let _x = 2;
    let y = 10;
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[test]
fn remaining_parts_of_value() {
    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {x}"),
    }
}

#[test]
fn extra_conditionals_with_match_guards() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => assert!(true),
        Some(x) => assert!(false),
        None => assert!(false),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

enum Message {
    Hello { id: i32 },
}

#[test]
fn fn_bindings() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Some other id: {id}"),
    }
}
