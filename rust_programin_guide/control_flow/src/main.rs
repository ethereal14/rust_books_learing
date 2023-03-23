use core::num;

fn main() {
    /* if 表达式 */
    /* 代码中的值必须是bool值，如果不是bool，报错 */
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    /* 不能这么写 */
    // if number{

    // }

    /* 判断非0 */
    if number != 0 {
        println!("number was something other than zero");
    }
    /* else if多重判断 */
    /* 只执行第一个条件为真的代码块 */
    let number = 6;
    /* 太多if else if就值得重构代码了 */
    /* C语言中, 我使用表格驱动法(元编程) */
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3,or 2");
    }
    println!("Hello, world!");

    /* 在let语句中使用if */
    let conditon = true;
    /* 有点像三元运算符 */
    /* 这种写法必须要求if分支返回的值都为同一个类型，不然报错 */
    let number = if conditon { 5 } else { 6 };
    /* let number = if conditon { 6 } else { "six" }; */
    println!("The value of number is:{}", number);

    /* 使用循环重复执行 */
    /* rust中可以使用break来退出循环 */
    /* continue 告诉程序跳过这个循环中的任何剩余代码, 进行下一个迭代 */
    /* 循环嵌套, break、countinue作用于最内层循环 */
    /* 可以在一个循环上指定一个循环标签, 然后将标签与break或者continue
    一起使用, 使这些关键字应用于已标记的循环, 而不是最内层循环 */
    let mut count = 0;
    /* 外层循环 */
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    /* loop返回值 */
    /* 重试可能会失败的操作 */
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("This result is {}", result);

    /* while 条件循环 */
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF");

    /* 使用for遍历集合 */
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    /* while版本 */
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    /* for版本 */
    /* 消除了数组越界，或遍历长度不够导致的bug */
    for element in a {
        println!("the value is: {}", element);
    }

    /* 使用 for 和 range 来循环倒计时 */
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

// TODO
// 华氏温度与摄氏度转换
// 生成n阶斐波拉契数列
// 打印圣诞歌(这个就别了)
