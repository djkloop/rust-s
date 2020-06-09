// fn main() {
//     let s1 = gives_ownership();
//     println!("s1 = {}", s1);

//     let s2 = String::from("Hrllo World");
//     let s3 = takes_and_gives_back(s2);
//     println!("s1 = {}", s3);
// }

// fn gives_ownership() -> String {
//     let s = String::from("Hello S1");
//     s
// }

// fn takes_and_gives_back(s: String) -> String {
//     s
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn modify_str(s: &mut String) {
    s.push_str(", World");
}
// 引用 (&)
// 指向值得引用，但是并不拥有它，因为拥有它这个值，所以，当引用离开指向值得作用域后也不会被丢弃
fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("len = {}, s1 = {}", len, s1);

    // 利用借用
    let mut s2 = String::from("Hello S2");
    modify_str(&mut s2);
    println!("s2 = {}", s2);

    //
    let mut m1 = String::from("HHHH");
    let mm1 = &m1;
    let mm2 = &m1;
    println!("{}, {}", mm1, mm2);
    let mm3 = &mut m1;
    mm3.push_str(", OOOO");
    println!("{}", mm3);
}
