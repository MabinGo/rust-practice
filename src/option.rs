pub fn test_option() {
    test_option1();
}

fn plus_one(x: Option<u8>) -> Option<u8> {
    match x {
        Some(i) => { Some(i + 1) }
        None => None
    }
}

fn test_option1() {
    let x = Some(1);
    println!("{:?}", plus_one(x));
    println!("{:?}", plus_one(None));
}