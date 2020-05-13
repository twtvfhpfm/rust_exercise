use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn message_test()
{
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![1,2,3,4,5];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![6,7,8,9,10];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let received = rx.recv().unwrap();
    println!("recvd: {}", received);

    for i in rx {
        println!("got: {}", i);
    }
}