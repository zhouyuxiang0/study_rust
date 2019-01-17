fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x value is {},y value is {}",x,y); //x value is 5,y value is 4
}
