use anyhow::Result;


// #[derive(Debug)]
// struct CusromErrorA {
//     err: CusromErrorB,
// }

// impl std::fmt::Display for CusromErrorA {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "CustomErrorA")
//     }
// }

// impl std::error::Error for CusromErrorA {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         Some(&self.err)
//     }
// }

// fn new_custom_error_a() -> Result<(), CusromErrorA> {
//     Err(CusromErrorA { err: CusromErrorB })
// }

// #[derive(Debug)]
// struct CusromErrorB;

// impl std::fmt::Display for CusromErrorB {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "CusromErrorB")
//     }
// }

// impl std::error::Error for CusromErrorB {}

// #[warn(dead_code)]
// fn new_custom_error_b() -> Result<(), CusromErrorB> {
//     Err(CusromErrorB)
// }

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    VarError(#[from] std::env::VarError),   
    #[error(transparent)]
    IoError(#[from] std::io::Error)    
}

fn main() -> Result<()> {
    // let s = std::fs::read_to_string("txt.txt")?;
    // println!("{}", s);
    // match new_custom_error_a() {
    //     Err(e) => {
    //         println!("Error: {}", e);
    //         println!("Caused by: {}", e.source().unwrap());
    //     }
    //     _ => println!("Ok"),
    // };
    // new_custom_error_b()?;
    // println!("aa");

//    let s =  std::env::var("DIR_PATH")?;
   let ss = std::fs::read_to_string("path")?;

    Ok(())
}
