pub fn test_pattern() {
    test_match22();// 绑定
    test_match21();//匹配守卫提供的额外条件
    test_match20();//用 .. 忽略剩余值
    test_match19();//_ 忽略部分值
    test_match18();//解构结构体和元组
    test_match17();//解构嵌套的结构体和枚举
    test_match16();//解构枚举
    test_match15();//解构结构体
    test_match14();//通过序列 ..= 匹配值的范围
    test_match13();//单分支多模式
    test_match12();//匹配命名变量
    test_match11();
    test_match10();
    test_match9();
    test_match8();
    test_match7();
    test_match6();
    test_match5();
    test_match4();
    test_match3();
    test_match2();
    test_match1();
}

enum MessageBinding {
    Hello { id: u32 }
}

fn test_match22() {
    let msg = MessageBinding::Hello { id: 5 };

    match msg {
        MessageBinding::Hello { id: id_variable @ 3..=7 } => { println!("Found an id in range: {}", id_variable) }
        MessageBinding::Hello { id: 10..=12 } => { println!("Found an id in another range") }
        MessageBinding::Hello { id } => { println!("Found some other id: {}", id) }
    }

    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    }

    // num 要绑定到所有的模式上 (1 | 2)，否则报错
    match 1 {
        num @ (1 | 2) => { println!("{}", num); }
        _ => {}
    }
}

fn test_match21() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn test_match20() {
    let t = (1, 2, 3, 4, 5);

    match t {
        (first, .., fifth) => { println!("first={},fifth={}", first, fifth); }
    }

    /* 下面的写法是错的，首先每个元组模式只能有一个..，其次不知道second该匹配哪个numbers
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
    */
}

fn test_match19() {
    // 可以在一个模式内部使用 _ 忽略部分值
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value.clone()) {
        (Some(_), Some(_)) => { println!("Can't overwrite an existing customized value"); }
        _ => { setting_value = new_setting_value; }
    }
    println!("setting is {:?}", setting_value);

    // 在一个模式中的多处使用下划线来忽略特定值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => { println!("Some numbers: {}, {}, {}", first, third, fifth) }
    }

    // _s 仍会将值绑定到变量，而 _ 则完全不会绑定
    let s = Some(2); //如果是非基础类型，比如String::from("Hello!") ，则会发生转移，编不过
    if let Some(_s) = s {
        println!("found a string");
    }
    println!("{:?}", s);
}

fn test_match18() {
    let ((feet, inches), Point { x, y }) = ((8, 9), Point { x: 1, y: 2 });

    //定长数组
    let arr: [u16; 2] = [0, 1];
    let [x, y] = arr;
    assert_eq!(x, 0);
    assert_eq!(y, 1);

    //不定长数组
    let arr: &[u16] = &[114, 514];
    if let [x, ..] = arr {
        assert_eq!(x, &114);
    }
    if let &[.., y] = arr {
        assert_eq!(y, 514);
    }
    let arr: &[u16] = &[];
    assert!(matches!(arr, [..]));
    assert!(!matches!(arr, [x, ..]));
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum MessageNested {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn test_match17() {
    let msg = MessageNested::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        MessageNested::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        MessageNested::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
        }
        _ => ()
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test_match16() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn test_match15() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 1, y: 8 };
    let Point { x, y } = p;
    assert_eq!(1, x);
    assert_eq!(8, y);

    let p = Point { x: 2, y: 9 };
    match p {
        Point { x, y: 2 } => println!("On the x axis at {}", x),
        Point { x: 2, y } => println!("On the y axis at {}", y), //x=2，匹配；y是任意值，匹配；
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

fn test_match14() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn test_match13() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn test_match12() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(10) => { println!("event: x={:?}", x); }
        Some(y) => { println!("event: y={}", y) }
        _ => { println!("no match") }
    };
    println!("end: x={:?},y={}", x, y); //y在match作用域中被修改，结束后回复原值
}

fn test_match11() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

// 变量遮蔽
fn test_match10() {
    let age = Some(30);
    println!("在匹配前，age是{:?}", age);

    if let Some(age) = age {
        println!("匹配出来的age是{}", age);
    }

    println!("在匹配后，age是{:?}", age);
}

fn test_match9() {
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));
}

#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar,
}

fn test_match8() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let _ = v.iter().filter(|x| matches!(x,MyEnum::Foo));
}

fn test_match7() {
    let value: Option<u8> = Some(2u8);//打桩
    let empty: Option<u8> = None; //打桩

    if let Some(i) = value {
        println!("Match number: {:?}", value);
    }

    if let Some(i) = empty {
        println!("Match number: {:?}", empty);
    } else {
        println!("Didn't match a number");
    }
}

#[derive(Debug)]
enum TraverseAll {
    First,
    Second,
    Third,
    Fourth,
}

fn test_match6() {
    let stages = [TraverseAll::First, TraverseAll::Second, TraverseAll::Third, TraverseAll::Fourth];

    for stage in stages {
        match stage {
            TraverseAll::First => { println!("First"); }
            TraverseAll::Second => { println!("Second"); }
            TraverseAll::Third => { println!("Third"); }
            x => { println!("{:?}", x) } //如果不穷尽匹配，编不过，可以用任意变量表示
            // _ => () //如果不穷尽匹配，编不过，可以用_表示
        }
    }
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn test_match5() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];

    for (_i, action) in actions.iter().enumerate() {
        match action {
            Action::Say(s) => { println!("{}", s); }
            Action::MoveTo(x, y) => { println!("point from (0, 0) move to ({}, {})", x, y); }
            Action::ChangeColorRGB(r, g, _) => { println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored", r, g); }
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin4 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}

fn test_match4() {
    let coin = Coin4::Quarter(UsState::Alaska);//打桩

    match coin {
        Coin4::Penny => 1,
        Coin4::Nickel => 5,
        Coin4::Dime => 10,
        //在匹配 Coin::Quarter(state) 模式时，它内部存储的值绑定到了 state 变量上，因此 state 变量就是对应的 UsState 枚举类型
        Coin4::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    };
}

enum IpAddr {
    Ipv4,
    Ipv6,
}

fn test_match3() {
    let ip1 = IpAddr::Ipv6;//打桩

    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn test_match2() {
    let coin = Coin2::Penny;//打桩

    match coin {
        Coin2::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter => 25,
    };
}

enum Direction {
    East,
    West,
    North,
    South,
}

fn test_match1() {
    let dire = Direction::South;//打桩

    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => { println!("South or North"); }//
        _ => println!("West"),
    };
}