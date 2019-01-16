use std::io; //要获取用户输入输出 需要使用io库
use std::cmp::Ordering; //引入该库 可使用Less Greater Equal
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        //使用变量存储用户输入 mutable(可变)
        let mut guess = String::new();

        //调用stdin函数 返回std::io::Stdin实例  若没有use std::io 则std::io::stdin
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        //创建一个immutabel的同名变量覆盖之前的mutabel变量
        // 用户输入时会使用回车此时 会在输入字符串后添加换行符 trim 消除字符串两侧空格 和换行
        //parse 字符串解析为数字
        let guess: u32 = match guess.trim().parse() {  //.expect("Please type a number!");输入非数字时用户可继续猜测
            Ok(num) => num,  //parse 返回Ok 或 Err 这里使用match 判断并处理
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        //使用cmp 比较 guess 和 secret_number  并返回 Less Greater 或 Equal 其一
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
