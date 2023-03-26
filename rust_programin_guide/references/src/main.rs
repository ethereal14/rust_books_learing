/* 引用的规则 */
/* 1. 在任意给定时间, 要么只能有一个可变引用, 要么只能有多个不可变引用 */
/* 2. 引用必须总是有效的(不要悬垂引用) */

fn main() {
    /* 与使用&引用相反的操作是解引用，使用运算符'*' */

    let s1 = String::from("hello");
    let len = calcultate_length(&s1);

    /* 创建引用的行为叫做借用，引用不能被修改 */
    // change(&s1);
    println!("The length of '{}' is {}.", s1, len);

    /* 可变引用 */
    let mut s = String::from("hello");
    println!("{}", s);
    change(&mut s);
    println!("{}", s);

    /* 可变引用同一时间只能有一个对某一特定数据的可变引用 */
    /* 尝试创建两个可变引用将会失败 */
    /* 好处是避免数据竞争:
     *      两个或更多指针同时访问统一数据
     *      只有有一个指针被用来写入数据
     *      没有同步数据访问的机制
     *  rust不能编译存在数据竞争的代码
     */
    let mut s = String::from("value");
    let r1 = &mut s;
    // let r2: &mut s;
    // println!("{}, {}", s1, s2);

    /* 可以使用大括号来创建一个新的作用域 */
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } /* r1 在这里离开了作用域, 所以我们可以再创建一个新的引用 */
    let r2 = &mut s;

    /* 类似规则也存在于同时使用可变与不可变引用中 */
    /* 不能在拥有不可变引用的同时拥有可变引用, 多个不可变引用是允许的 */
    let mut s = String::from("value");
    let r1 = &s; /* 没问题 */
    let r2 = &s; /* 没问题 */
    // let r3 = &mut s; /* 大问题 */
    // println!("{}, {}, {}", r1, r2, r3);

    /* 一个引用的作用域从声明的地方开始到最后一次使用它为止 */
    /* 例如: begin*/
    let mut s = String::from("value");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
    /* end */
    /* ******************** */
    /* 悬垂引用 */
    /* 悬垂指针: 它指向的内存可能已经分配给其他人用了 */
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    // fn dangle() -> &String {/* dangle 返回一个字符串引用 */
    let s = String::from("value"); /* s是一个星字符串 */

    // &s/* 返回字符串s的引用 */
    /* 最好直接返回字符串 */
    s
} /* 这里, s离开作用域并被丢弃, 其内存被释放, 危险! */

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calcultate_length(s: &String) -> usize {
    s.len()
} /* 这里，s离开了作用域，但是它并不拥有引用值的所有权，
  所以什么也不会发生 */
