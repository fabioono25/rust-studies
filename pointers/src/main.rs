use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

// use Nil
use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5); // allocated on the heap
    println!("b = {}", b);

    // working with cons list. Vec<T> is a better choice
    let _l1 = (1, (2, (3, Nil)));

    // use the enum List below
    // let _l2 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // working with Drop
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // working with Rc<T>
    // let aa = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let bb = Cons(3, Rc::clone(&aa));
    // let cc = Cons(4, Rc::clone(&aa));
    // println!("count after creating bb = {}", Rc::strong_count(&aa));
    // {
    //     let dd = Cons(4, Rc::clone(&aa));
    //     println!("count after creating dd = {}", Rc::strong_count(&aa));
    // }

    // working with RefCell<T>
    // let x = 5;
    // let y = &mut x; // cannot borrow `x` as mutable, as it is not declared as mutable

    // let value = Rc::new(RefCell::new(5));

    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // *value.borrow_mut() += 10;

    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
