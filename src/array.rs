use::std::io;

pub fn test_array() {
    test_array4();
    test_array3();
    test_array2();
    test_array1();
}

fn test_array4() {
    // 编译器自动推导出one的类型
    let one = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let slice: &[u8] = &two[1..3];
    assert_eq!(slice, &[2, 3]);

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    // 借用arrays的元素用作循环中
    for a in &arrays {
        println!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            println!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += &a[i];
        }
        println!("\t({:?} = {})", a, sum);
    };
}

fn test_array3() {
    // let array = [String::from("rust is good!"); 8]; //出错，因为非基础类型不能用Copy trait赋值
    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    println!("{:?}", array);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn test_array2() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();
    // 读取控制台的输出
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = &a[index];

    println!("The value of the element at index {} is: {}", index, element);
}

fn test_array1() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; //包含 5 个元素，这些元素的初始化值为 3
}