pub fn test_generics() {
    test_generics6(); //在该结构体的方法中定义额外的泛型参数，就跟泛型函数一样
    // test_generics5(); //在方法中使用泛型
// test_generics4(); //std的Some、Ok、Err都是泛型
    // test_generics3(); //定义多个泛型
// test_generics2(); //泛型示例
    // test_generics1(); //泛型示例
}

struct PointThree<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointThree<T, U> {
    fn mixup<V, W>(self, other: PointThree<V, W>) -> PointThree<T, W> {
        PointThree {
            x: self.x,
            y: other.y,
        }
    }
}

fn test_generics6() {
    let p1 = PointThree { x: 5, y: 10.4 };
    let p2 = PointThree { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

struct PointTwo<T> {
    x: T,
    y: T,
}

impl<T> PointTwo<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn test_generics5() {
    let p = PointTwo { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

// enum Option<T> {
//     Some(T),
//     None,
// }
//
// enum Result<T, E> {
//     Ok(T),
//     //例：如果打开文件成功，返回Ok(std::fs::File)，因此 T 对应的是 std::fs::File 类型
//     Err(E),//例：如果打开文件失败，返回Err(std::io::Error)，E 对应的就是 std::io::Error 类型
// }

fn test_generics4() {
    Some(0i8);
    // Ok::<i8, E>(0i8);
    // Err::<i8, E>(1i8);
}

struct PointOne<T, U> {
    x: T,
    y: U,
}

fn test_generics3() {
    let p = PointOne { x: 1, y: 1.1 };
}

/*
fn largest<T: for<'a> std::cmp::PartialOrd<&'a T>>(list: &[T]) -> T {
    let mut largest = &list[0];

    for &item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

fn test_generics2() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
*/

// 不是所有 T 类型都能进行相加操作，因此我们需要用 std::ops::Add<Output = T> 对 T 进行限制
fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

fn test_generics1() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));
}