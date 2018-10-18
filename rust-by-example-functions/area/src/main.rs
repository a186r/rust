// 方法是附加到对象上的函数，使用impl定义，这些方法可以通过self访问对象的参数和其他方法。

struct Point {
    x: f64,
    y: f64,
}

// 实现方法的代码块，为Point实现具体的方法
impl Point {
    // 这是一个静态方法
    // 静态方法不需要由实例调用
    // 这些方法通常用作构造函数
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // 另一个带有两个参数的静态方法
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // 这是一个实例方法
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // abs是一个'f64'的方法，返回调用者的绝对值
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // 这个方法要求调用方法的对象是可变的
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destory(self) {
        let Pair(first, second) = self;

        println!("Destroying Pair({},{})", first, second);
    }
}

fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pari = Pair(Box::new(1), Box::new(2));
    pari.destory();
}
