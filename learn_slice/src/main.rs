fn main() {
    let s = String::from("Hello World");

    let _h = &s[0..5];
    let _h = &s[0..=4];
    let _h = &s[..=4];
    let h = &s[..5];
    println!("h = {}", h);

    let _w = &s[6..];
    let _w = &s[6..=10];
    let _w = &s[6..];
    let w = &s[..];
    println!("w = {}", w);

    //  let ss = String::from("你好");
    //  let w1 = &ss[0..1];
    //  println!("w1 = {}", w1);

    let arr = [1, 2, 3, 4];
    let s = &arr[1..3];
    println!("s = {}", s[0]);
    println!("s = {}", s[1]);
    println!("len = {}", s.len());
}
