// #[cfg(test)]
// mod tests {
//     // 这个属性表明这是一个测试函数，这样测试执行者就知道要将其作为测试处理
//     #[test]
//     fn exploration() {
//         // 函数体通过使用assert_eq!来断言
//         assert_eq!(2 + 2, 4);
//     }

//     #[test]
//     fn another(){
//         panic!("make this test fail");
//     }
// }

// 使用assert!宏来检查结果

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }
}