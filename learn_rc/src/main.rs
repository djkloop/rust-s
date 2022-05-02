use std::rc::Rc;
use crate::List::{Cons, Nil};

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // let x = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // 共享内存
    let x = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let _y = Cons(3, Rc::clone(&x));
    let _y = Cons(3, x.clone());
    // let _z = Cons(4, Rc::clone(&x));
    let _z = Cons(4, x.clone());
    println!("Hello, world!");
}
