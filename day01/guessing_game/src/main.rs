// 通知rust我们要使用外部依赖，这也会调用相应的use rand，所以现在可以使用rand::前缀来调用rand crate中的任何内容
extern crate rand;

// 输入输出库，获取用户输入
// io库来自于标准库(也被称为std)
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

// 获取1-101之间的随机数
    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is:{}",secret_number);

    loop {
    
        println!("Please input your guess.");

    // 在rust中，变量默认是不可变的，在变量名前使用mut来使一个变量可变
    // new是string类型的一个关联函数，new函数在这里创建了一个空的string，这是创建类型实例的惯用函数名
        let mut guess = String::new();

    // 同理，stdin就是io库的关联函数，调用stdin，调用read_line方法从标准输入句柄获取用户输入

    // 我们还向read_line()传递了一个参数：&mut guess

    // read_line的工作是无论用户输入什么内容，都将其存入一个字符串中，因此它需要字符串作为参数，这个参数应该是
    // 可变的，以便将用户的输入附加上去。

    // &表示这个参数是一个引用，允许多处代码访问同一处数据，而无需在内存中多次拷贝

    // 下半部分是另一个方法，最好拆成单行来写。

    // 使用Result类型来处理潜在的错误，Result类型是枚举，通常也写作enums，如果不写下面那行，程序可以正常执行，
    // 但是会抛出一个警告，说明可能有一个潜在的错误没有解决，所以应该编写错误处理代码，如果我们就是希望出现错误时
    // 程序立即奔溃，所以直接使用expect
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

    // trim消除前后的空格，消除用户按下回车之后的\n
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

    // 使用println!占位符打印值，{}是预留的占位符，第一个{}对应第一个值，第二个对应第二个值，以此类推
        println!("You guessed:{}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}