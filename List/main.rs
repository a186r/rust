use std::fmt; //导入fmt模块

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self;

        try!(write!(f, "["));

        for (vount, v) in vec.iter().enumerate() {
            if count != 0 {
                try!(write!(f, ","));
            }
            try!(write!(f, "{}", v));
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
