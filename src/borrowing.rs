pub fn test_borrowing() {
    test_borrowing3();
    test_borrowing2();
    test_borrowing1();
}

fn test_borrowing3() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    let r4 = &mut s;

    //只是声明而不用，新编译器不报错，用才报错，加下面这段就报错了
    // println!("{}{}{}{}", r1, r2, r3, r4);
}

fn test_borrowing2() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn test_borrowing1() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}