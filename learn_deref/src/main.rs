use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

// 实现解引用的trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn print_hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    //
    let z = Box::new(x);
    assert_eq!(5, *z);

    //
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    //
    let m = MyBox::new(String::from("Rust"));
    // &String -> &str
    // 解引用多态
    print_hello(&m);

    println!("Hello, world!");
}
