// 迭代器

trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>; // Self::Item 关联类型
}
fn main() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter(); // 迭代器
    // for val in v1_iter {
    //     println!("val => {}", val);
    // }

    if let Some(v) = v1_iter.next() {
        println!("v => {}", v);
    }

    if let Some(v) = v1_iter.next() {
        println!("v => {}", v);
    }

    if let Some(v) = v1_iter.next() {
        println!("v => {}", v);
    }

    if let Some(v) = v1_iter.next() {
        println!("v => {}", v);
    } else {
        println!("v => None");
    }

    // 迭代可变引用
    let mut v2 = vec![1,2,3];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next() {
        *v = 3;
    }
    println!("v2 => {:?}", v2);

    // 消费适配器
    let v3 = vec![1,2,3];
    let v3_iter = v3.iter();
    let total: i32 = v3_iter.sum(); // 求和
    println!("total => {}", total);

    // 迭代适配器
    let v1 = vec![1,2,3];
    let v2: Vec<i32> = v1.iter().map(|x| x+1).collect();
    println!("v1 => {:?}", v1);
    println!("v2 => {:?}", v2);

    // filter
    let v1 = vec![1,2,3,12,45];
    let v2: Vec<i32> = v1.into_iter().filter(|x| *x > 5).collect();
    println!("v2_filter => {:?}", v2);

    println!("Hello, world!");
}
