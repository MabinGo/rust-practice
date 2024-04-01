pub fn test_procctrl() {
    test_procctrl5();
    test_procctrl4();
    test_procctrl3();
    test_procctrl2();
    test_procctrl1();
}

fn test_procctrl5() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn test_procctrl4() {
    for _ in 0..10 {
        //循环执行10次
    }

    // 第一种,
    // (1)collection[index] 的索引访问，会因为边界检查导致运行时的性能损耗 —— Rust 会检查并确认 index 是否落在集合内
    // (对 collection 的索引访问是非连续的，存在一定可能性在两次访问之间，collection 发生了变化，导致脏数据产生)
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        let item = &collection[i];
        // ...
    }

    // 第二种，
    // (1)直接迭代的方式就不会触发这种检查，因为编译器会在编译时就完成分析并证明这种访问是合法的
    // (2)直接迭代的方式是连续访问，因此不存在这种风险( 由于所有权限制，在访问过程中，数据并不会发生变化)
    for item in collection {}
}

fn test_procctrl3() {
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}

fn test_procctrl2() {
    for i in 1..=5 {
        println!("{}", i);
    }
}

fn test_procctrl1() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}