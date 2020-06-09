// 1. 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
// 2. 引用必须有效

fn main() {
    let refs = dangle();
    println!("refs = {}", refs);
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}
