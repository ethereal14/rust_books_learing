/* 函数 */
/* fn 函数名() {} */
/* fn 开始, 后跟着函数名和一对圆括号, 大括号告诉编译器函数体开始和结束 */
fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    println!("Hello, world!");

    let y = 6; /* 语句 statement 并不返回值 */
    // let x = (let y = 6);/* 运行时错误 */
    /* c中可以写: x = y = 6; rust中不可以这么写!!!!! */

    let y = {
        let x = 3;
        x + 1 /* 没有分号, 加了分号这里转化为语句;表达式的结尾没得分号!! */
    };
    println!("The value of y is:{}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

/* 参数: 函数添加参数 */
/* 在函数签名中必须声明每个参数的类型，多个参数时使用逗号分割 */
fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {} {}", value, unit_label);
}

/* 表达式 */
/* let y = 6 6是一个表达式；宏调用也是表达式；
函数调用也是表达式；大括号包裹的也是表达式 */

/* 代返回值的函数 */
/* 在圆括号后边指明函数返回值类型 */
fn five() -> i32 {
    /* 函数没有参数并定了返回值类型,
    函数主体只有5,也没有分号，所以这是一个表达式 */
    5
}
/* 不加分号, 运行没问题 */
fn plus_one(x: i32) -> i32 {
    x + 1
}

/* 加分号运行, 报错 */
// fn plus_one(x: i32) -> i32 {
//     x + 1;
// }

/* 注释 */

/* 文档注释 */
// 单行注释
