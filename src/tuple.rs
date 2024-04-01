pub fn test_tuple() {
    test_tuple2();
    test_tuple1();
}


fn tup_calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}

fn test_tuple2() {
    let s1 = String::from("hello");
    let (s2, len) = tup_calculate_length(s1);

    // println!("The length of {}", s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn test_tuple1() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    println!("tup value is: {},{},{}", tup.0, tup.1, tup.2);
}