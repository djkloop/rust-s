struct Stepper {
    cur: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.max {
            return None;
        }
        let res = self.cur;
        self.cur += self.step;
        Some(res)
    }
}

fn main() {
    let address = 0o22;
    let mut list = vec![1, 2, 5, 15, 42, 132, 3412, 16, 18];
    for item in list.iter_mut() {
        if *item == address {
            println!("{}", item);
            *item = 3333;
        }
    }
    println!("{:?}", list);

    let ex = Stepper {
        cur: 2,
        step: 3,
        max: 15,
    };

    for item in ex {
        println!("{}", item);
    }
}
