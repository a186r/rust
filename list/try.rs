use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref aec) = *self;

        try!(write!(f, "["));

        // 对vec进行迭代,v是每次迭代的值,count是迭代次数
        for (count, v) in aec.iter().enumerate() {
            // 在调用write前，对每个元素加上逗号
            if count != 0 {
                try!(write!(f, ","));
            }
            try!(write!(f, "{}", v));
        }
        //加上中括号，并且返回fmt:Result的值
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
