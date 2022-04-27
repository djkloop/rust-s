// 闭包
fn main() {
    let use_closure = || {
        println!("this is a closure");
    };
    use_closure();

    fn add_one_v1(x: i32) -> i32 {
        x + 1
    }

    // 闭包可以自动推到，但是只能推到一次！
    let add_one_v2 = |x: i32| -> i32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;

    let a = add_one_v1(5);
    let b = add_one_v2(5);
    let c = add_one_v3(5);
    let d = add_one_v4(5);

    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);

    //
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    println!("s = {}", s);

    // 报错! 不能推导两次
    // let n = example_closure(5);
    // println!("n = {}", n);

    // 捕捉环境中的变量
    let i = 1;
    let exe = |x| x + i;
    let r = exe(5);
    println!("r = {}", r);

    println!("Hello, world!");
}
