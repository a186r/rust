// // // use std::thread;
// // // use std::time::Duration;

// // // fn main() {

// // //     // 创建一个新线程需要调用thread::spawn函数并传递一个闭包，其包含希望在新线程中运行的代码
// // //     // 从thread::spawn保存一个JoinHandle，以确保该线程能够运行至结束。
// // //     let handle = thread::spawn(|| {
// // //         for i in 1..10{
// // //             println!(
// // //                 "hi number {} from the spawned thread!",
// // //                 i
// // //             );
// // //             // 调用强制线程停止执行一小段时间，这会允许其他线程运行
// // //             thread::sleep(Duration::from_millis(1));
// // //         }
// // //     });

// // //     // 如果将handle的join放在这，主线程会等待知道新线程执行完毕之后才开始
// // //     handle.join().unwrap();

// // //     for i in 1..5{
// // //         println!(
// // //             "hi number {} from the main thread!",
// // //             i
// // //         );

// // //         thread::sleep(Duration::from_millis(1));
// // //     }

// // // // 通过调用handle的join，会阻塞当前线程直到handle所代表的线程结束。
// // //     // handle.join().unwrap();
// // // }

// // // ---------------------------------------------------------线程与move闭包
// // use std::thread;

// // fn main(){
// //     let v = vec![1,2,3];


// //     // println!("Here's other vector: {:?}",
// //             // v);
// // // 闭包尝试借用v，但是不知道v的引用是否一直有效
// // // 通过在闭包之前增加move关键字，强制闭包获取其使用的值的所有权，而不是任由rust推断它应该借用值。
// //     let handle = thread::spawn(move || {
// //         println!(
// //             "Here's a vector: {:?}",
// //             v
// //         );
// //     });


// //     handle.join().unwrap();
// // }

// // use std::sync::mpsc;

// // fn main(){
// //     // 创建一个通道，并将其两端赋值给tx和rx
// //     let(tx,rx) = mpsc::channel();
// //     // tx.send(()).unwrap();
// // }

// use std::thread;
// use std::sync::mpsc;

// fn main(){

//     let (tx,rx) = mpsc::channel();

//     // 使用闭包，move将tx移动到子线程内
//     thread::spawn(move || {
//         let val = String::from("h");
//         // 通道的发送端使用send方法来获取需要放入通道的值
//         tx.send(val).unwrap();
//     });

// // 在主线程中接收
//     let received = rx.recv().unwrap();

//     println!(
//         "Got: {}",
//         received
//     );
// }

// -----------------------------通道的所有权转移
// use std::thread;
// use std::sync::mpsc;

// fn main(){
//     let (tx,rx) = mpsc::channel();

//     thread::spawn(move || {
//         // 尝试在通过tx.send发送val到通道中之后将其打印出来。但是
//         // 一旦将值发送到另一个线程之后，那个线程可能在我们再次使用它之前就将其修改或者丢弃。
//         let val = String::from("hello");
//         tx.send(val).unwrap();
//         println!(
//             "val is {}",
//             val
//         );
//     });

//     let received = rx.recv().unwrap();
//     println!(
//         "Got: {}",
//         received
//     );
// }

// ---------------------------------------------------------发送多个值并观察接收者的等待
// use std::thread;
// use std::sync::mpsc;
// use std::time::Duration;

// fn main(){
//     let (tx,rx) = mpsc::channel();

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         // 发送多个消息，并在每次发送之后暂停一段时间
//         for val in vals{
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {}",received);
//     }
// }

// ---------------------------通过clone发送者来创建多个生产者
