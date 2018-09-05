### 基础知识点 -- 2周

《Rust by Example》，把其中的例子实现一遍，了解rust常用概念
- 类型
- 变量
- 流程控制
- 函数
- 模块
- 库
- 属性
- 泛型
- 作用域
- 特性
- 错误处理
- 标准库

《The Rust Programming Language》，进一步理解rust语言某些细节
- 所有权
- 枚举与模式匹配
- 模块、通用集合
- 迭代器与闭包
- 指针、并发
- 高级特性

### cita源码 -- 2周
- 阅读cita国密算法版源码，理解其中的rust实现
https://github.com/cryptape/cita/releases/download/v0.18/cita_sm2_sm3.tar.gz

### parity中使用国密


### 实现共识算法

1.一周学习基础语法
列出问题

2.了解cita的国密算法
库、椭圆签名，hash

parity现在椭圆曲线算法，hash函数怎么用的

3.了解现在poa的原理
poa作为一个独立的工作模块，怎么和parity其他模块(通信、账本、消息处理等)打交道的
列出来相关的接口，自己测一下，做验证

4.基于bft的共识算法 ibft tendermint了解，关注下个测试版
ibft规范