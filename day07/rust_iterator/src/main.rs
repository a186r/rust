fn main() {

    // let v1 = vec![1,2,3,4];

    // let v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got: {}",val);
    // }

    // 调用迭代器适配器map来创建一个新迭代器.
    // let v1: Vec<i32> = vec![1,2,3];

    // v1.iter().map(|x| x + 1);

    // -------------------------------------------产生迭代器的方法

    let v1: Vec<i32> = vec![1,2,3];

    v1.iter().map(|x| x + 1);


}

trait Iterator{
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// #[test]
// fn iterator_demonstration() {
//     let v1 = vec![1,2,3,4,5];

//     let mut v1_iter = v1.iter();

//     assert_eq!(v1_iter.next(),Some(&1));
//     assert_eq!(v1_iter.next(),Some(&2));
//     assert_eq!(v1_iter.next(),Some(&3));
//     assert_eq!(v1_iter.next(),None);
// }

#[test]
fn iterator_sum() {
    let v1 = vec![1,2,3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}


// 调用map方法创建一个新迭代器,接着调用collect方法消费新迭代器并创建一个vector
#[test]
fn iterator_sum_1(){
    let v1: Vec<i32> = vec![1,2,3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2,vec![2,3,4]);
}

// 在迭代器上直接调用next方法
#[test]
fn iterator_demonstration(){
    let v1 = vec![1,2,3];

// 这里v1_iter是可变的，因为调用next方法会改变迭代器中用来记录序列位置的状态
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(),Some(&1));
    assert_eq!(v1_iter.next(),Some(&2));
    assert_eq!(v1_iter.next(),Some(&3));
    assert_eq!(v1_iter.next(),None);
}

// ------------------------------------------------------消费迭代器的方法
#[test]
fn iterator_sum2(){
    let v1 = vec![1,2,3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

