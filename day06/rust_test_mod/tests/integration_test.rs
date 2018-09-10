// // 每个tests目录中的文件都被编译为单独的crate，因此要单独导入库
// extern crate rust_test_mod;

// #[test]
// fn it_adds_two(){
//     assert_eq!(4,rust_test_mod::add_two(2));
// }

// ------------------------------------------集成测试中的子模块
// 将每个集成测试文件当做其自己的crate来对待，更有助于创建的单独的作用域
extern crate rust_test_mod;
// 引入模块
mod common;

#[test]
fn it_adds_two(){
    // 上面引入了模块，这里就可以调用setup方法了
    common::setup();
    assert_eq!(4,rust_test_mod::add_two(2));
}