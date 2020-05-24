const MAX_POIONT: u32 = 10000;
fn main() {
    // 1. 变量定义
    // 定义变量用let，如果变量没有使用mut，那么变量是不可变的。
    let a = 1;
    println!("a = {}", a);

    let mut b: u32 = 2;
    println!("b1 = {}", b);
    b = 4;
    println!("b2 = {}", b);

    // 2. 隐藏性
    let b: f32 = 1.1;
    println!("b = {}", b);

    // 3. 常量
    println!("MAX_POINT = {}", MAX_POIONT);
}
