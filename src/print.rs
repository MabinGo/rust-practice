pub fn test_print() {
    test_print1();
    test_derive();
}

struct TAG<T> {
    a: T,
    b: T,
}

impl<T> TAG<T> {
    fn func(&self) -> &T {
        &self.a
    }
}
fn test_print1() {
    let ins = TAG { a: 1, b: 2 };
    println!("{}", ins.func());
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:<width$}", number = 1, width = 6);
    println!("My name is {0}, {1} {0}", "Bond", "Bill");
    println!("{}", format!("Hello"));
    println!("{}", format!("{:?}", (3, 4)));
}

fn test_derive() {
    #[derive(Debug)]
    struct Structure(i32);
    println!("This struct `{:?}` ", Structure(3));

    #[derive(Debug)]
    struct Deep(Structure);
    println!("This struct `{:#?}` ", Deep(Structure(10)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }
    let name = "Peter";
    let age = 20;
    let peter = Person { name, age };
    println!("{:#?}", peter);
}