use std::ops::Deref;

// tuple struct 
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn hello(name: &str) {
    println!("Hello, {}!", name);
}

pub fn test_deref() {
    let b = Box::new(5);
    println!("b = {}", b);

    let x = 5;
    // let y = Box::new(x);

    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let name = Box::new(String::from("Rust"));
    // 这里使用 &m 调用 hello 函数，其为 MyBox<String> 值的引用。
    // 因为在 MyBox<T> 上实现了 Deref trait，Rust 可以通过 deref 调用将 &MyBox<String> 变为 &String。
    // 标准库中提供了 String 上的 Deref 实现，其会返回字符串 slice。Rust 再次调用 deref 将 &String 变为 &str，这就符合 hello 函数的定义了。
    hello(&name);
    // 如果 Rust 没有实现 Deref 强制转换，为了使用 &MyBox<String> 类型的值调用 hello，则需使用以下代码：
    hello(&(*name)[..]);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn test_drop() {
    let _c = CustomSmartPointer {
        data: String::from("Hello world"),
    };
    // std::mem::drop  prelude
    drop(_c);
    let _d = CustomSmartPointer {
        data: String::from("你好世界"),
    };
    println!("CustomSmartPointers created.");
}

fn main() {
    // test_deref();
    test_rc();
}

use List::Cons;
use List::Nil;
use std::rc::Rc;

enum List {
    Cons (i32, Rc<List>),
    Nil,
}

pub fn test_rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating a = {}", Rc::strong_count(&a));

    {
        // let c = Cons(4, Rc::clone(&a));
        let _c = Cons(4, a.clone()); // 同上面注释代码
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

