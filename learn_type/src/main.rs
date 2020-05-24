fn main() {
    // bool
    let is_true: bool = true;
    println!("is_true = {}", is_true);
    let is_false: bool = false;
    println!("is_false = {}", is_false);
    // rust自带类型推导
    let is_bool = true;
    println!("is_bool = {}", is_bool);

    // char 在rust里面，char是32位的
    let char_a = "a";
    println!("char_a = {}", char_a);
    let char_b = "你";
    println!("char_b = {}", char_b);

    // i8, i16, 132, i64, u8, u16, u32, u64, f32, f64
    let i8_c: i8 = -111;
    println!("i8_c = {}", i8_c);
    let f32_d = 0.00000008;
    println!("f32_b = {}", f32_d);

    // 自适应类型 isize, usize
    println!("max = {}", usize::max_value());

    // 数组 [Type; size]
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0] = {}", arr[0]);
    let arr1: [u32; 3] = [1, 2, 3];
    show(arr1);

    // 元组
    let tup: (i32, f32, char) = (-3, 3.2, '你');
    println!("{}", tup.0);
    // 自带类型推导
    let tup = (-3, 3.2, '你');
    println!("------------------");
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    println!("------------------");

    // 有点类似js的解构赋值
    let (x, y, z) = tup;
    println!("------------------");
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
    println!("------------------");
}

fn show(arr1: [u32; 3]) {
    println!("------------------");
    for i in &arr1 {
        println!("{}", i);
    }
    println!("------------------");
}
