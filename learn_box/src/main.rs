use crate::List::Nil;
use List::Cons;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // b -> 存储在栈上，5存在堆上
    // b指向5的内存地址
    let b = Box::new(5);
    println!("b => {}", b);

    //
    let _ = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Hello, world!");
}
