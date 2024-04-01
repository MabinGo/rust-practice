pub fn test_structure() {
    test_struct4();
    test_struct3();
    test_struct2();
    test_struct1();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn test_struct4() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

struct AlwaysEqual;

// impl SomeTrait for AlwaysEqual {
//
// }
fn test_struct3() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

#[derive(Debug)]
struct FileStruct {
    name: String,
    data: Vec<u8>,
}

fn test_struct2() {
    let f1 = FileStruct {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:#?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}

#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u32,
}

fn test_struct1() {
    let user1 = User {
        email: String::from("someone@rust-practice.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        // active: user1.active,
        // username: user1.username,
        // email: String::from("another@rust-practice.com"),
        // sign_in_count: user1.sign_in_count,
        ..user1
    };

    println!("{}", user1.active);
    // 下面这行会报错,赋值时username、email已被转移到user2， bool和u64等类型实现了Copy特征，所以没发生所有权转移
    // println!("{:#?}", user1);
    println!("{:#?}", user1.sign_in_count);
}