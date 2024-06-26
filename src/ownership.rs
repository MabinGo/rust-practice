pub fn test_ownership() {
    test_ownership5();
    test_ownership4();
    test_ownership3();
    test_ownership2();
    test_ownership1();
    test_danglingref1();
}

fn dangle() -> String {
    let s = String::from("hello");
    s //所有权转移。如果写&s，s被释放后就没用了
}

fn test_danglingref1() {
    let reference_to_nothing = dangle();
}

fn test_ownership5() {
    let s1 = gives_ownership();         // gives_ownership 将返回值移给 s1
    let s2 = String::from("hello");     // s2 进入作用域
    let s3 = takes_and_gives_back(s2);  // s2 被移动到takes_and_gives_back 中,它也将返回值移给 s3

    println!("{}{}", s1, s3);
}

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域.
    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}

fn test_ownership4() {
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里 ...... 所以到这里不再有效

    let x = 5;                      // x 进入作用域
    makes_copy(x);
    println!("x={}", x);
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn test_ownership3() {
    let x: &str = "hello, world";
    let y = x;

    println!("x={}, y={}", x, y);
}

fn test_ownership2() {
    let x = 5;
    let y = x;

    println!("x={}, y={}", x, y);
}

fn test_ownership1() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
}