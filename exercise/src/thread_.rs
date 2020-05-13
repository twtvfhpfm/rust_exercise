use std::thread;
use std::time::Duration;

pub fn thread_test()
{
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }

        println!("here's a vec {:?}", v);
    });

    for i in 1..3 {
        println!("main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join();

}