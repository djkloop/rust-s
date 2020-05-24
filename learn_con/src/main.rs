fn main() {
    // if
    let y = 0;
    if y == 1 {
        println!("y = 1");
    }

    // if - else
    if y == 1 {
        println!("y = 1")
    } else {
        println!("y != 1")
    }

    // if - else - if - else
    if y == 1 {
        println!("y = 1")
    } else if y == 0 {
        println!("y == 0")
    } else if y == 3 {
        println!("y == 3")
    } else {
        println!("other")
    }

    let condition = true;
    // if - else 里面的所有类型都应该一致
    // let x = if condition { 5 } else { "6" }; error
    let x = if condition { 5 } else { 6 };
    println!("x = {}", x);

    // loop
    let mut counter = 0;
    loop {
        println!("in loop");
        if counter == 3 {
            break;
        };
        counter += 1;
    }

    let result = loop {
        counter += 1;
        if counter == 6 {
            break counter * 3;
        }
    };
    println!("result = {}", result);

    // while
    let mut i = 0;
    while i != 10 {
        i += 1;
    }
    println!("i = {}", i);

    // for
    let arr = [1, 2, 3, 4];
    // for i in arr.iter() {
    for i in &arr {
        println!("element = {}", i);
    }
}
