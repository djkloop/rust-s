// 静态生命周期

use std::fmt::Display;

fn func<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("ann is {}", ann);
    if x.len() < y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("s1");
    let s2 = String::from("s2, hello");
    let ann = 129;
    let r = func(s1.as_str(), s2.as_str(), ann);
    println!("r = {}", r);
    println!("Hello, world!");
}
