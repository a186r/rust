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
// ------------------------------------------------------并行或者连续的运行测试
// 当运行多个测试时，他们默认使用线程来并行的运行，这意味着测试会更快的运行完毕，因为测试是并行的，
// 所以你需要确保测试不能相互依赖，或者依赖共享的环境，比如当前工作目录或者环境变量