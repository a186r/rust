// 单元结构体
struct Nil;

// 元祖结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main(){
    // 实例化结构体
    let point：Point = Point{x:0.3,y:0.4};

    // 访问point的字段
    println!("point cordinates: ({},{})",point.x,point.y );

    // 使用let绑定来结构point
    let Point{x:my_x,y:my_y} = point;

    let _rectangle = Rectangle{
        p1:Point{x:my_y,y:my_x};
        p2:point;
    }
}


// PW5JNwZSv1owEKqpxs9rH6qPAP5htiqB9nrdF8ry4PWuCMcMKFgyo
// Private key: 5JzwkbZo36Nu7RJhcPeUtnwWv3tE6AeoMSiVN6RApL9oKPrsKe9
// Public key: EOS4twE46pdwVh54ESuShhTpaNSSPaVXhY9GiMTVkHSBGVupbMoT1
interest1990