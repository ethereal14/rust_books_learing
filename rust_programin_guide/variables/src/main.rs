use std::io;

fn main() {
    let mut x = 5;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5; /* 变量的遮蔽 */
    let x = x + 1;
    {
        /* 内部作用域 */
        let x = x * 2;
        println!("The value of x in the inner scope is :{}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len(); /* ok, 遮蔽可以改变数据类型 */

    let mut spaces = "   ";
    // spaces = spaces.len(); /* error, mut变量不可更改数据类型, 将得到编译器错误 */
    let x = 2.0; //f64
    let y: f32 = 3.0; //f32

    /* 数字运算 */
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    let remainder = 43 % 5;

    /* 布尔类型 */
    let t = true;
    let f: bool = false;

    /* 字符类型 */
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    /* 复合类型 */
    /* 元组类型 */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; /* 解构元组 */
    println!("The value of y is:{}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    /* 没有任何值的元组是一种特殊类型, 只有一个值。
    该类型称为单元类型，该值称为单元值 */

    /* 数组类型 */
    /* 不确定使用数组还是vector, 那就使用vector */
    /* 针对数组元素不变的情况, 使用数组 */
    let a = [1, 2, 3, 4, 5];
    let month = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];
    /* 使用方括号编写数组的类型, 其中包含元素类型‘分号，然后是元素数 */
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    /* 初始化数组的另一种写法 */
    let a = [3; 5]; /* [元素组;数组长度] */

    /* 访问数组 */
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    /* 无效数组访问 */
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];/* 访问越界时, 直接报运行时错误 */
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
