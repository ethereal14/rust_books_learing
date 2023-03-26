/*
 * 另一个没有所有权的数据类型是slice,
 * slice允许引用集合中一段连续的元素序列,
 * 而不用引用整个集合
 */

fn main() {
    let mut s = String::from("Hello, world!");

    let word = first_word(&s); /* word的值为5 */
    s.clear(); /* 这里清空了字符串, 使其等于"" */

    /* word 在此处的值任为5 */
    /* 但是没有更多的字符串让我们可以有效的应用数值5,word现在完全无效 */
    /* 如果后续要返回一个开始的索引和结尾索引, 就会导致有三个飘忽不定的变量 */
    /* rust提供了: 字符串slice */

    let s = String::from("hello world");
    /* 不是对整个string引用 */
    let hello = &s[0..5];
    let world = &s[6..11];

    /* .. range语法: 索引从0开始 */
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    /* 对于最后一个字节 */
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    /* 两个都舍弃 */
    let slice = &s[0..len];
    let slice = &s[..];

    /* slice range的索引必须在有效的utf8字符边界捏
     * 如果尝试从一个多字节字符的中间位置创建字符串slice, 报错
     */

    /* slice改造first_word */
    let mut s = String::from("hello world");
    let word = first_word_new(&s);
    // s.clear();/* error  */
    println!("the first word is: {}", word);

    /* 字符串字面量就是slice */
    /* 这里 s 的类型是 &str 它是一个指向二进制程序特定位置的slice
     * 所以字符串字面量是不可变的; &str 是一个不可变引用
     */
    let s = "hello world!";

    let my_string = String::from("hello world");
    /* first_word 接收 String 的切片, 无论是部分还是全部 */
    let word = first_word_last(&my_string[0..6]);
    let word = first_word_last(&my_string[..]);
    /* first_word 也接受String的引用 */
    /* 等同于String的全部切片 */
    let word = first_word_last(&my_string);

    let my_string_literal = "hello world";

    /* first_word_last 接受字符串字面量的切片 */
    let word = first_word_last(&my_string_literal[0..6]);
    let word = first_word_last(&my_string_literal[..]);

    /* 因为字符串字面量就是字符串slice */
    let word = first_word_last(&my_string_literal);

    /* 其他类型slice */
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

/* 更好的版本: 可以对&str、String操作 */
/* 如果有一个字符串slice, 可以直接传递它; 如果有一个String
 * 则可以传递整个String的slice或对String的引用.
 */
fn first_word_last(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word_new(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word(s: &String) -> usize {
    /* 因为需要逐个元素检查String中的值是否为空格, 需要用as_bytes
     * 方法将String转化为字节数
     */
    let bytes = s.as_bytes();

    /* 使用iter方法在字节数组上创建一个迭代器 */
    /* enumerate包装这些返回结果, 将这些元素作为元组的一份来返回 */
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
