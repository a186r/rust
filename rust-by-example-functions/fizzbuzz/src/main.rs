// 不像C、C++，rust对于函数的定义顺序没有限制

fn main() {
    fizzbuzz_to(100);
}

// 返回布尔值的function
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    lhs % rhs == 0
}

// 函数返回()实际上表示的是不返回任何值
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// 当函数返回()时，可以从函数签名中省略返回类型
fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}
