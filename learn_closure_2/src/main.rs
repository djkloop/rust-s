//
struct Cacher<T>
where
    T: Fn(i32) -> i32,
{
    caculation: T,
    value: Option<i32>,
}

impl<T> Cacher<T>
where
    T: Fn(i32) -> i32,
{
    fn new(caculation: T) -> Cacher<T> {
        Cacher {
            caculation,
            value: None,
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.caculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let mut c = Cacher::new(|x| x+1);
    let v1 = c.value(1);
    println!("v1 = {}", v1);

    //
    // let x = 4;
    // let equal_to_x = |z| z==x;
    // let y = 4;
    // assert!(equal_to_x(y));

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z==x;
    // move 掉了所有权 不能在使用x
    // println!("x = {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    println!("Hello, world!");
}
