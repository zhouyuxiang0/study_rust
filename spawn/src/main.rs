use std::sync::mpsc;
use std::time::Duration;
use std::thread;

fn main() {
    // 建立一个由发送者和接受者的通道
    // tx发送者 rx 接收者 通过解构 mpsc 多个producer single consumer
    let (tx, rx) = mpsc::channel();

    // 克隆通道的发送端
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
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
