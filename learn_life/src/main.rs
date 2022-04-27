// 函数的生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}

// fn a_str<'a>(x: &'a str, y: &'a str) -> &'a str {
//     let r = String::from("hello");
//     r.as_str()
// }

fn main() {
    let s1 = String::from("abcdef");
    let s2 = String::from("ab");
    let r = longest(s1.as_str(), s2.as_str());
    println!("r = {}", r);

    let ss = get_str(s1.as_str(), s2.as_str());
    // let ss2 = a_str(s1.as_str(), s2.as_str());
    println!("Hello, world!");
}
