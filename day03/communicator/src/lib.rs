// 使用pub修饰就可以消除未被使用的警告了
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        // 从跟模块开始
        // ::client::connect();
        // 或者直接使用super上移到当前模块的父模块
        // super::client::connect();
        // 或者直接使用use super::client
        client::connect();
        assert_eq!(2 + 2, 4);
    }
}

// 最后总结一下模块文件系统的规则
// 1.如果一个叫foo的模块没有子模块，应该将foo的声明放入叫做foo.rs的文件中。
// 2.如果一个叫foo的模块有子模块，应该将foo的声明放入叫做foo/mod.rs的文件中。


// 私有性规则
// 1.如果一个项是公有的，它能被任何父模块访问
// 2.如果一个项是私有的，它能被其直接父模块及其任何子模块访问
