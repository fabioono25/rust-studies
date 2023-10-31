// use Nil
use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5); // allocated on the heap
    println!("b = {}", b);

    // working with cons list. Vec<T> is a better choice
    let _l1 = (1,(2,(3, Nil)));
    
    // use the enum List below
    let _l2 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
