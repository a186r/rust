// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

pub fn add_two(a:i32) -> i32{
    internal_adder(a,2)
}

fn internal_adder(a:i32,b:i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn internal(){
        // assert_eq!(4,internal_adder(2,2));
        assert_eq!(5,add_two(3));
    }

}