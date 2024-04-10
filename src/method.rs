pub fn test_method() {
    test_memthod3(); // 定义枚举的方法
    // test_memthod2(); // 方法名跟结构体的字段名相同
    // test_memthod1(); //自处理方法、关联函数定义和调用
}

#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

fn test_memthod3() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn test_memthod2() {
    let rect1 = Rectangle { width: 30, height: 50 };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

#[derive(Debug)]
struct Circle {
    x: u32,
    y: u32,
    reserved: u32,
}

impl Circle {
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    fn new(x: u32, y: u32, reserved: u32) -> Self {
        Circle { x, y, reserved }
    }

    // Circle的方法，&self表示借用当前的Circle结构体
    fn selfproccess(&self) {
        println!("self processing, current value is:x={},y={},reserved={}", self.x, self.y, self.reserved);
    }
}

fn test_memthod1() {
    let c = Circle::new(1, 2, 3);
    c.selfproccess();
    // let c = c.new(1, 2, 3);
}
