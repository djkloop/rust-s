fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn makes_copy(i: i32) {
    println!("i = {}", i * 5);
}

fn main() {
    // 引用类型，无法在后面的地方使用
    let s = String::from("Hello");
    let mut result_s1 = takes_ownership(s);
    result_s1.push_str(" World");
    println!("{}", result_s1);
    // 值类型
    let x = 5;
    makes_copy(x);
    println!("x = {}", x);
}
