// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

// pub struct AveragedCollection{
//     list: Vec<i32>,
//     average: f64,
// }

// // 在结构体上实现了add、remove、average等方法
// impl AveragedCollection{
//     pub fn add(&mut self,value: i32){
//         self.list.push(value);
//         self.update_average();
//     }

//     pub fn remove(&mut self) -> Option<i32>{
//         let result = self.list.pop();
//         match result {
//             Some(value) => {
//                 self.update_average();
//                 Some(value)
//             },
//             None => None,
//         }
//     }

//     pub fn average(&self) -> f64{
//         self.average
//     }

//     fn update_average(&mut self){
//         let total: i32 = self.list.iter().sum();
//         self.average = total as f64 / self.list.len() as f64;
//     }
// }

// pub trait Draw{
//     fn draw(&self);
// }

// 定义一个pub的Screen结构体，带有components字段
pub struct Screen {
    pub components: Vec<Box<Draw>>,
}

impl Screen{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

// pub struct Screen<T: Draw>{
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
//     where T: Draw{
//         pub fn run(&self){
//             for component in self.components.iter(){
//                 component.draw();
//             }
//         }
//     }

// ----实现trait

pub trait Draw{
    fn draw(&self);
}

pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// 一个实现了Draw trait的Button结构体
impl Draw for Button{
    fn draw(&self){

    }
}