use std::ops::Deref;

#[cfg(test)] // test only when you run cargo test
mod tests {
    use core::prelude::v1;

    use super::*;

    #[test]
    fn box_like_reference() {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn customized_smart_pointer() {
        let x = 5;
        // let y = MyBox::new(x);
        let y = MyBox(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}


struct MyBox<T>(T); // tuple struct

impl<T> Deref for MyBox<T> {

    type Target = T; // associated type

    fn deref(&self) -> &Self::Target {
        &self.0
    }

    // fn new(x: T) -> MyBox<T> {
    //     MyBox(x)
    // }
}