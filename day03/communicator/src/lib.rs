mod client;
mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// 最后总结一下模块文件系统的规则
// 1.如果一个叫foo的模块没有子模块，应该将foo的声明放入叫做foo.rs的文件中。
// 2.如果一个叫foo的模块有子模块，应该将foo的声明放入叫做foo/mod.rs的文件中。