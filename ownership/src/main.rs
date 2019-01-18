// 所有权的存在 是为了管理堆数据
// Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
// 值有且只有一个所有者。
// 当所有者（变量）离开作用域，这个值将被丢弃。
fn main() {
    // rust 在整型赋值时 因为是已知固定大小 会将值分配在栈中
    let x = 5;
    // 这里会生成一个x值得拷贝 并赋值给y 现在就有两个变量x,y 都等于 5
    let y = x;

    // 一个指向存放字符串内容内存的指针，一个长度，和一个容量。这一组数据存储在栈上
    // 堆上存放的是字符串值
    let s1 = String::from("hello");
    // 这里会生成一个s1的拷贝(prt,len,capacity) 但不会在堆中生成字符串值得拷贝
    // 这时rust为了避免在内存回收时出现两次回收,s1将被rust认为无效，因此s1离开作用域不需要清理任何东西
    let s2 = s1;
    //如果想要深拷贝 可以使用 let s2 = s1.Clone();
    let mut s = String::from("hello");  // 从此处起 s 是有效的

    s.push_str(", world!"); // push_str() 在字符串后面追加字面值

    println!("{}", s);
} // 此作用域已结束
//s 不在有效

//rust 会在结尾}处自动调用drop的特殊函数