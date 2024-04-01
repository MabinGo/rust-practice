pub fn test_basetype() {
    test_mutvar();
    test_destructuring();
    test_const();
    test_shadow();
    test_typederive();
    test_string1();
    test_composittype1();
}

#[allow(unused_variables)]
type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

fn test_composittype1() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    //read(&mut f1, &mut vec![]);
    close(&mut f1);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn say(p0: String) {
    println!("{}", p0);
}

fn test_string1() {
    let my_name = "Pascal";
    let name = String::from("This is String type");
    greet(my_name);
    say(name.clone());
    println!("{}", my_name);
    println!("{}", name);
}


fn test_typederive() {
    let guess: i32 = "42".parse().expect("Not a number!");
    let guess = "42".parse::<i32>().expect("Not a number!");
}

fn test_shadow() {
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽，结果12
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    //结果6
    println!("The value of x is: {}", x);

    // 字符串类型
    let spaces = "   ";
// usize数值类型
    let spaces = spaces.len();
}

fn test_const() {
    const MAX_POINTS: u32 = 100_000; //对数字字面量可插入下划线以提高可读性
    println!("1:{:?}", MAX_POINTS);
}

struct Struct {
    e: i32,
}

fn test_destructuring() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e,..
} = Struct { e: 5 };

    assert_eq ! ([1, 2, 1, 4, 5], [a, b, c, d, e]);
    }

fn test_mutvar() {
    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}