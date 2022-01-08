trait Person {
    fn new(name: String) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {
        println!("hello world!")
    }
}

struct APersion {
    name: String,
}

impl Person for APersion {
    fn new(name: String) -> Self {
        APersion { name }
    }

    fn language(&self) -> &str {
        "APersion"
    }
}

struct BPersion {
    name: String,
}

impl Person for BPersion {
    fn new(name: String) -> Self {
        Self { name }
    }

    fn language(&self) -> &str {
        "BPersion"
    }

    fn say_hello(&self) {
        println!("BPerson")
    }
}

fn main() {
    let a = APersion::new("APerson".to_string());
    let b = BPersion::new("BPersion".to_string());
    println!("{}", a.name);
    println!("{}", b.name);
    a.say_hello();
    b.say_hello();
}
