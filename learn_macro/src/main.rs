trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_as_bytes {
    ($($type: ty),*) => {
        $(
            impl AsBytes for $type {
                fn as_bytes(&self)->Vec<u8> {
                    Vec::from(<$type>::to_ne_bytes(*self))
                }
            }
        )*
    };
}

macro_rules! yo {
    ($name: expr) => {
        let x = $name;
        println!("{}", x);
    };
    ($($item:expr),*) => {
        $(
            let x = $item;
            println!("{}", x);
        )*
    }
}

mod marcos {
    #[macro_export]
    macro_rules! macro1 {
        () => {
            println!("1");
            macro1!(@macro2);
        };
        (@macro2) => {
            println!("hello");
        }
    }
}

fn main() {
    yo!(1);
    yo!(1, 2, 3, 4);

    impl_as_bytes!(u16, u32);
    let u32_item = 1287u32;
    println!("bytes: {:?}", u32_item.as_bytes());
    macro1!();
}
