fn main() {
    println!("Hello, world!");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let result = largest(&number_list);
    println!("The largest number (using generics) is {}", result);

    let result = largest(&char_list);
    println!("The largest char is (using generics) {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let string = Point { x: "Hello", y: "World" };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

// without generics
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// without generics
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// now, using generics in function
fn largest<T>(list: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
    let mut largest = &list[0];

    for item in list {
        // error: the trait `std::cmp::PartialOrd` is not implemented for `T`
        if item > largest {
            largest = item;
        }
    }

    largest
}

// multiple generic type parameters
struct Point<T, U> {
    x: T,
    y: U,
}

// enum Option<T> {
//     Some(T),
//     None,
// }

// using generics in method definitions
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}