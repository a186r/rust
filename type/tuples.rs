use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    // 这里是省略return
    (boolean, integer)
}

fn transpose(pair: (f32,f32) -> (f32,f32)){
    let (integer,integer) = pair;
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{}) \n({},{})", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // 通过元组的索引来访问具体的值
    println!("lont_tuple first value : {}", long_tuple.0);
    println!("lont_tuple second value : {}", long_tuple.1);

    // 元组也可以充当元组的元素
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 打印元组
    println!("tuple of tuples : {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("the reverse pair is {:?}", reverse(pair));

    // 创建单元素元组需要一个额外的逗号，就是为了和括号包含的普通数据区分
    println!("one element tuple : {:?}", (5u32,));
    println!("just an integer : {:?}", (5u32));

    // 解构元组，将值赋给创建的绑定变量
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?},{:?},{:?},{:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix)
}
