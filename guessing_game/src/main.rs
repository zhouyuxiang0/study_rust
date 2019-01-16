use std::io; //要获取用户输入输出 需要使用io库

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();  //使用变量存储用户输入 mutable(可变)

    io::stdin().read_line(&mut guess)   //调用stdin函数 返回std::io::Stdin实例  若没有use std::io 则std::io::stdin
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
