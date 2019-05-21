use std::sync::mpsc;
use std::time::Duration;
use std::thread;

fn main() {
    // tx发送者 rx 接收者 通过解构 mpsc 多个producer single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
