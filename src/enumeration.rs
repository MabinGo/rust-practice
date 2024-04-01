pub fn test_enumeration() {
    test_enumeration4();
    test_enumeration3();
    test_enumeration2();
    test_enumeration1();
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn test_enumeration4() {
    let five = Some(5);
    println!("{:?}", five);

    let six = plus_one(five);
    println!("{:?}", six);

    let none = plus_one(None);
    println!("{:?}", none);
}

// Option<T> 枚举是如此有用以至于它被包含在了 prelude之中，你不需要将其显式引入作用域
/*
#[derive(Debug)]
enum Option<T>{
    Some(T),
    None
}
*/
fn test_enumeration3() {
    let some_number = Some(5);
    println!("{:?}\n", some_number);

    let some_string = Some("a string");
    println!("{:?}\n", some_string);

    let absent_number: Option<i32> = None;
    println!("{:?}\n", absent_number);
}

/*
fn new (stream: TcpStream) {
    let mut s = stream;
    if tls {
        s = negotiate_tls(stream)
    }

    // websocket是一个WebSocket<TcpStream>或者
    //   WebSocket<native_tls::TlsStream<TcpStream>>类型
    websocket = WebSocket::from_raw_socket(
        s, ......)
}
*/
enum Websocket {
    // Tcp(Websocket<TcpStream>),
    // Tls(Websocket<native_tls::TlsStream<TcpStream>>),
}

fn test_enumeration2() {}

enum Msg {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test_enumeration1() {
    let q = Msg::Quit;
    let m = Msg::Move { x: 1, y: 2 };
    let w = Msg::Write(String::from("123"));
    let c = Msg::ChangeColor(0, 1, 2);
}