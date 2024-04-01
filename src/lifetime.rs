use std::fmt::Display;

pub fn test_lifetime() {
    test_lifetime9();
    test_lifetime8();
    test_lifetime7();
    test_lifetime6();
    test_lifetime5();
    test_lifetime4();
    test_lifetime3();
    test_lifetime2();
    test_lifetime1();
}

fn longest_with_an_announcement1<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test_lifetime9() {
    let x = longest_with_an_announcement1("123", "456", 1);
    println!("{}", x);
}


fn test_lifetime8() {
    let s: &'static str = "我没啥优点，就是活得久，嘿嘿";
}

struct ImportantExcerpt3<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt3<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
        where
            'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}

struct ImportantExcerpt2<'a> {
    part: &'a str,
}

impl<'a: 'b, 'b> ImportantExcerpt2<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn test_lifetime7() {
    let x = ImportantExcerpt2 {
        part: "123"
    };
    let y = ImportantExcerpt3 {
        part: "456"
    };

    println!("{}", x.announce_and_return_part("abc"));
    println!("{}", y.announce_and_return_part("xyz"));
}

struct ImportantExcerpt1<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt1<'a> {
    fn level(&self, _x: &str) -> &str {
        &self.part
    }
}

fn test_lifetime6() {
    let s = ImportantExcerpt1 {
        part: "123456",
    };
    println!("{}", s.level("123"));
}

fn first_wood(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    };

    &s[..]
}

fn test_lifetime5() {
    let s = first_wood("123 abc");
    println!("{:#?}", s);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn test_lifetime4() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}

fn longest2(_x: &str, _y: &str) -> String {
    // 返回内部字符串的所有权，然后把字符串的所有权转移给调用者
    String::from("really long string")
}

fn test_lifetime3() {
    let s = longest2("not", "important");
    println!("{}", s);
}

// 在通过函数签名指定生命周期参数时，我们并没有改变传入引用或者返回引用的真实生命周期，而是告诉编译器当不满足此约束条件时，就拒绝编译通过
fn longest1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test_lifetime2() {
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = longest1(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
}

fn test_lifetime1() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest1(string1.as_str(), string2);
    println!("The longest string is {}", result);
}