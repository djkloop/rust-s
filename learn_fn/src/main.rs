// 不传参数
fn other_fun() {
    println!("This is a function");
}

// 传参数(参数必须要带类型)
fn other_fun1(a: i32, b: u32) {
    println!("a = {}, b = {}", a, b);
}

// 返回带类型的
fn other_fun2(a: i32, b: i32) -> i32 {
    let result = a + b;
    return result;
}

// 返回带类型的 可以不写return但是不能写冒号
fn other_fun3(a: i32, b: i32) -> i32 {
    // let result = a + b;
    // result 这样也可以哦~
    a + b
}

fn main() {
    //
    other_fun();
    //
    let a = -1;
    let b = 2;
    other_fun1(a, b);
    //
    let c = 9;
    let r = other_fun2(a, c);
    println!("r1 = {}", r);
    //
    let r = other_fun3(a, c);
    println!("r2 = {}", r);
    // let y = 1; // 语句， 没有返回值
    let y = {
        let x = 1;
        x + 1
    };
    println!("y = {}", y);

    println!("Hello, world!");
}
