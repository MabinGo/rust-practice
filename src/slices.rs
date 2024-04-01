pub fn test_slice() {
    test_slice4();
    test_slice3();
    test_slice2();
    test_slice1();
}

fn test_slice4() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

fn test_slice3() {
    let s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {}", word);
}

fn test_slice2() {
    let s = "中国人";
    let a = &s[..3];
    let b = &s[3..6];
    let c = &s[6..];
    println!("{}\n{}\n{}\n", a, b, c);
}

fn test_slice1() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{},{}", hello, world);

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[2..len];
    println!("{}", slice);
    let slice = &s[..2];
    println!("{}", slice);
}