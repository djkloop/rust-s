/*
 * @Author       : djkloop
 * @Date         : 2021-07-20 23:28:46
 * @LastEditors  : djkloop
 * @LastEditTime : 2021-07-21 00:14:23
 * @Description  : 头部注释
 * @FilePath     : /gussing_game/src/main.rs
 */
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数字！");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("神秘数字是：{}", secret_number);

    loop {
        println!("在输入一个数字！");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个正确的数字");
                continue;
            },
        };

        println!("最终获得的数字之和是：{}", guess + secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
