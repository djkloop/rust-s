struct Persion<'a> {
    name: &'a str,
    age: i32,
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let man = Persion {
        name: "djkloop",
        age: 30,
    };
}
