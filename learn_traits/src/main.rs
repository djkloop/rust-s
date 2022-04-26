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

/// trait_bound
trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge {
    fn get_age(&self) -> u32;
}

// 写法1.
// fn print_infomation<T: GetName + GetAge>(info: T) {
//     println!("name = {}", info.get_name());
//     println!("age = {}", info.get_age());
// }

use std::fmt::Debug;

// 写法2.
fn print_infomation<T>(info: T)
where
    T: GetName + GetAge,
{
    println!("name = {}", info.get_name());
    println!("age = {}", info.get_age());
}

#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub age: u32,
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}

#[derive(Debug)]
pub struct Teacher {
    pub name: String,
    pub age: u32,
}

impl GetName for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }
}

//
fn produce_info_with_age() -> impl GetAge + Debug {
    Student {
        name: "xiaoming".to_string(),
        age: 18,
    }
}

//
fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}

fn main() {
    let a = APersion::new("APerson".to_string());
    let b = BPersion::new("BPersion".to_string());
    println!("{}", a.name);
    println!("{}", b.name);
    a.say_hello();
    b.say_hello();
    // trait bound
    let s = Student {
        name: "小明".to_string(),
        age: 10,
    };
    print_infomation(s);
    let s_age = produce_info_with_age();
    println!("{:#?}", s_age);

    //
    let number_list = vec![1, 2, 3, 412, 4, 62, 21];
    let max_number = largest(&number_list);
    println!("最大值 -> {}", max_number);
}
