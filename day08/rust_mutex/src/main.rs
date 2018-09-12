// use std::sync::Mutex;
// use std::thread;

// // 在线程间共享，将启动是个线程，并在各个线程中对同一个计数器值加一
// // 这里使用Mutex<T>在多个线程间共享值
// fn main(){
//     let counter = Mutex::new(0);
//     let mut handles = vec![];

//     // for _ in 0..10{
//     //     let handle = thread::spawn(move || {
//     //         let mut num = counter.lock().unwrap();

//     //         *num += 1;
//     //     });

//     //     handles.push(handle);
//     // }
//     let handle = thread::spawn(move || {
//         let mut num = counter.lock().unwrap();

//         *num += 1;
//     });
//     handles.push(handle);

//     let handle2 = thread::spawn(move || {
//         let mut num2 = counter.lock().unwrap();

//         *num2 += 1;
//     });
//     handles.push(handle2);

//     for handle in handles{
//         handle.join().unwrap();
//     }

//     println!("Result: {}",*counter.lock().unwrap());
// }

// 原子性
use std::sync::{Mutex,Arc};
use std::thread;

fn main(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!(
        "Result: {}",
        *counter.lock().unwrap()
    );
}