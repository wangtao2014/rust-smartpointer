# rust-smartpointer
- 引用
    - Rust 中最常用的指针是 引用（reference）
    - 引用以 & 符号为标志并借用了他们所指向的值
    - 除了引用数据没有任何其他特殊功能


- 智能指针
    - 是一类数据结构
    - 行为和指针相似
    - 有额外的元数据和功能
    - Rust 标准库中不同的智能指针提供了多于引用的额外功能
        - 引用计数 （reference counting） 智能指针类型，其允许数据有多个所有者。
        - 引用计数智能指针记录总共有多少个所有者，并当没有任何所有者时负责清理数据。
    - 引用：只借用数据
    - 智能指针：拥有它所指向的数据

- Rust 需要在编译时知道类型占用多少空间
- deref trait
    - 实现 deref 方法
    - 该方法借用 self
    - 返回一个指向内部数据的引用

- 函数或方法的隐式解引用转换
    - 为函数或方法提供的一种便捷特性
    - Deref 强制转换只能作用于实现了 Deref trait 的类型。
    - Deref 强制转换将这样一个类型的引用转换为另一个类型的引用

## Deref 强制转换如何与可变性交互
```
类似于如何使用 Deref trait 重载不可变引用的 * 运算符，Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符。

Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：

当 T: Deref<Target=U> 时从 &T 到 &U。
当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
当 T: Deref<Target=U> 时从 &mut T 到 &U。
头两个情况除了可变性之外是相同的：

第一种情况表明如果有一个 &T，而 T 实现了返回 U 类型的 Deref，则可以直接得到 &U。
第二种情况表明对于可变引用也有着相同的行为。
第三个情况有些微妙：Rust 也会将可变引用强转为不可变引用。但是反之是 不可能 的：不可变引用永远也不能强转为可变引用。因为根据借用规则，如果有一个可变引用，其必须是这些数据的唯一引用（否则程序将无法编译）。将一个可变引用转换为不可变引用永远也不会打破借用规则。将不可变引用转换为可变引用则需要初始的不可变引用是数据唯一的不可变引用，而借用规则无法保证这一点。因此，Rust 无法假设将不可变引用转换为可变引用是可能的。
```

## drop trait
- 当值将要离开作用域时发生的动作
- 任何类型都可以实现 drop trait
- 比如：文件、网络资源释放等
- 只要实现 drop 方法
    - 对 self 的可变引用
    - 在 prelude 模块中