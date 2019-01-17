fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is:{}", element);
    }

    for number in (1..4).rev() { //rev() 反转range
        println!("{}", number);
    }
}
