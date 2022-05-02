struct Dog {
    name: String,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("Dog {} leave", self.name);
    }
}

struct Cat {
    name: String,
}

impl Drop for Cat {
    fn drop(&mut self) {
        println!("Cat {} leave", self.name);
    }
}

fn main() {
    let a = Dog {
        name: "a".to_string(),
    };
    {
        let b = Dog {
            name: "b".to_string(),
        };
        println!("b => {}", b.name);
    }

    println!("a => {}", a.name);

    // 提前drop
    // 官方提供了对应的方法
    let _aa = Cat {
        name: "aa".to_string(),
    };
    let bb = Cat {
        name: "bb".to_string(),
    };
    // 如果使用了drop， 那么打印hello world的时候, bb变量已经被释放掉了, 会提前答应drop事件
    drop(bb);
    println!("Hello, world!");
}
