use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Teminate,
}

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender}
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self){
        for _ in &self.workers {
            self.sender.send(Message::Teminate).unwrap();
        }
        for worker in &mut self.workers {
            println!("shut down worker({})", worker.id);

            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("worker {} got a job, executing", id);
                    job();
                },
                Message::Teminate => {
                    println!("worker {} was told to terminate", id);
                    break;
                }
            }
        });
        Worker{id, thread: Some(thread)}
    }
}
