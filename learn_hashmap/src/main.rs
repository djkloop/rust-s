use std::collections::HashMap;

fn main() {
    let mut string_map = HashMap::new();
    string_map.insert("张三", 12);
    string_map.insert("李四", 15);
    println!("{:#?}", string_map);
    match string_map.get(&"张三") {
        Some(&age) => println!("张三年龄为{}", age),
        _ => {}
    }
    string_map.remove(&"张三");
    for (name, age) in string_map.into_iter() {
      println!("{}, {}", name, age);
    }
}
