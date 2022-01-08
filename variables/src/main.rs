/*
 * @Author       : djkloop
 * @Date         : 2021-07-21 00:17:31
 * @LastEditors  : djkloop
 * @LastEditTime : 2021-07-21 01:04:41
 * @Description  : 头部注释
 * @FilePath     : /variables/src/main.rs
 */

const MAX_POINTS: u32 = 10_000;

fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    println!("The Const value of x is {}", MAX_POINTS);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let guess: u32 = "42".parse().expect("转义错误");
    println!("{}", guess);

    let x = 2.0;
    let y: f32 = 9.0;
    println!("{}, {}", x, y);

    let x = 'z';
    let y: char = '≈';
    let z = '😄';
    println!("{}, {},{}", x, y, z);

    let tup: (i32, f64, u8) = (500, 48.22, 25);
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let a = [1, 2, 34, 56, 7];
    println!("{}", a[1]);

    func_one();

    func_two(88);

    let y = {
        let x = 1;
        x + 3
    };

    println!("value -> {}", y);

    let x = func_five(11);
    println!("val -> {}", x);

    let x = 3;
    // if x % 4 == 0 {
    //     println!("v -> 4");
    // } else if x % 3 == 0 {
    //     println!("v -> 3");
    // } else {
    //     println!("not 3 or 4");
    // }
    // match x {
    //    (x % 4 == 0) =>  println!("v -> 4"),
    // }

}

fn func_one() {
    println!("aaaa");
}

fn func_two(v: i32) {
    println!("v -> {}", v);
}

fn func_five(x: i32) -> i32 {
    6 + x
}
