#[derive(Debug)]
struct A<'a> {
    name: &'a str,
}

// 省略生命周期
fn get_a_str(s: &str) -> &str {
    s
}

// 方法生命周期
struct StuA<'a> {
    name: &'a str,
}

impl<'a> StuA<'a> {
    fn do_something(&self) -> i32 {
        3
    }

    fn do_something2(&self, s: &str) -> &str {
        self.name
    }

    fn do_something3<'b>(&self, s: &'b str) -> &'b str {
        s
    }
} 

fn main() {
    let n = String::from("hello");
    let a = A { name: &n };
    println!("a => {:?}", a);

    //
    let s = get_a_str(&n);
    println!("s => {:?}", s);

    //
    let s = String::from("hello");
    let sa = StuA { name: &s };
    println!("sa => {:?}", sa.do_something());
    let s2 = String::from("hello");
    println!("sa2 => {:?}", sa.do_something2(&s2));
    println!("sa3 => {:?}", sa.do_something3(&s2));

    println!("Hello, world!");
}
