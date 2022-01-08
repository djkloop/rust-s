fn main() {
    let cat = "cat";
    println!("{}", cat);
    let dog = String::new();
    println!("''-> {}", dog);
    let dog = String::from("dog");
    println!("form -> {}", dog);
    let dog = "dog_string".to_string();
    println!("tostring -> {}", dog);
    let mut dog = format!("{}", "你好");
    println!("{}", dog);
    println!("{}", dog.len());
    dog.push('!');
    println!("format -> {}", dog);
    let result = dog.replace('你', "说");
    println!("rep -> {}", result);
}
