fn main() {
    let s = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("{}", s);
    println!("Hello, world!");

    /* 如果在c中，这玩意叫浅拷贝, 但是在这儿叫做"移动" */
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1);/* 被移动了, s1已经被回收 */
    /* 变量与数据的交互: 克隆 */
    /* 确实需要深拷贝, 可以使用clone的函数 */
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    /* 只在栈上的数据: 拷贝 */
    /*
     * 栈上数据大小是编译期确定下来的,
     * 所以克隆的很快, 这儿直接clone了
     * 不再是move
     */
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    /* *
     * rust有一个叫做Copy trait的的标注, 可以用在类似整型这样存储在
     * 栈上的类型。如果一个类型实现了Copy trait, 那么一个旧的变量将
     * 其值赋给其他变量后任然可用。
     *
     * rust不允许实现了Drop trait的类型使用copy trait。
     *
     * 任何标量类型都支持copy
     *      所有整型
     *      布尔类型
     *      浮点类型
     *      字符类型
     *      元组: 当且仅当包含的类型都实现了copy, 才能copy
     *           (i32, i32) --> copy ok
     *           (i32, String) --> copy no
     * */

    let s = String::from("hello"); /* s 进入作用域 */
    takes_ownership(s); /* s的值"移动"到这里 */

    // println!("{}", s);/* 所以这里s已经无效 */
    let x = 5; /* x进入作作用域 */
    make_copy(x); /* x移动到函数 */
    println!("x = {}", x); /* 但是 i32 应该可以是copy的,所以这里可以继续使用 */

    /* 返回值与作用域 */
    /* 返回值也可以转移所有权 */
    let s1 = gives_ownership(); /* gives_ownership 将返回值移动到s1 */

    let s2 = String::from("hello"); /* s2进入作用域 */

    /* s2被移动到tacks_and_gives_back中, 也将返回值移动到s3 */
    let s3 = takes_and_gives_back(s2);

    /* 示例4-5 */
    /* 变量的所有权: 将值赋给另一个变量时移动它。当持有所有权的变量离开作用域时
     * 将其值通过drop被清理掉, 除非数据被移动为另一个变量所有
     *
     * 在每一个函数中都获取所有权并接着返回太过于啰嗦。
     * 我们想要函数使用一个值但不获取所有权: 传进去再返回出来, 对于多个数据可以是用元组返回
     *
     * 这样太过形式主义: 另外一种机制: references 引用
     */
    let s1 = String::from("hello");
    let (s2, len) = calculate_lenth(s1);
    println!("The length of '{}' is {}.", s2, len);
} /* 这里s3移除作用域并被丢弃, s2 也被移除作用域, 但已被移走, 所以什么也不会发生 */
/* s1移出作用域并被丢弃 */

fn takes_ownership(some_string: String) {
    /* some_string进入作用域 */
    println!("{}", some_string);
} /* 这里, some_string 溢出作用域并调用'drop'方法，使用占用内存*/

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
} /* 这里 some_integer移除作用域。不会有特殊操作 */

fn gives_ownership() -> String {
    /* gives_ownership将返回值移动给调用他的函数 */
    let some_string = String::from("yours");

    some_string /* 返回some_string并移出给调用的函数 */
}
/* takes_and_gives_back将传入字符串并返回该值 */
fn takes_and_gives_back(a_string: String) -> String {
    /* a_string进入作用域 */
    a_string /* 返回 a_string并移出给调用的函数 */
}

fn calculate_lenth(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
