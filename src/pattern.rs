pub fn test_pattern() {
    test_match7();
    // test_match6();
    // test_match5();
    // test_match4();
    // test_match3();
    // test_match2();
    // test_match1();
}

fn test_match7() {
    let value = Some(2u8);
    if let Some(2) = value {
        println!("Second");
    }
    /* 等价于下面
    let value = Some(2u8);
    match value {
        // Some(s) => { println!("output is: {}", s) }
        Some(2) => {println!("output is: {:?}", Some(2)) },
        _ => (),
    };
    */
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
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored", r, g);
            }
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
    let coin = Coin4::Quarter(UsState::Alaska);

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
    let ip1 = IpAddr::Ipv6;
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
    let coin = Coin2::Penny;
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

/*
match target {
模式1 => 表达式1,
模式2 => {
语句1;
语句2;
表达式2
},
_ => 表达式3
}
*/

enum Direction {
    East,
    West,
    North,
    South,
}

fn test_match1() {
    let dire = Direction::South;

    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => { println!("South or North"); }//
        _ => println!("West"),
    };
}