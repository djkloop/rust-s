fn main() {
    let mut arr = vec![1; 20];
    arr.push(2);
    println!("{:?}", arr);
    arr.remove(0);
    println!("{:?}", arr);
    match arr.get(21) {
        Some(item) => println!("{}", item),
        None => {}
    }
}