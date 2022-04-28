//! My Crate
//! 
//! 'my_crate' is a collection of utilities to make performing certain calculations more convenient.
//! 
/// Add one to the number given.
/// # Example
/// ```
/// let five = 5;
/// 
/// assert_eq!(6, learn_doc_comments::add_one(5));
/// 
/// ```
pub fn add_one(x:i32) -> i32 {
    x + 1
}




// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
