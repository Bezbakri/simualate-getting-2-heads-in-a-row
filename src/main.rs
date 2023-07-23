use rand::random;
use std::thread;
use std::sync::{mpsc, mpsc::Receiver};
use std::thread::JoinHandle;
use std::time::Instant;
use std::io;

pub fn counter() -> f64{
    let mut sum: f64 = 0.0;
    let range: i32 = 1000000;
    for _ in 0..range {
        let mut last: bool = false;
        let mut count: f64 = 0.0;
        loop {
            count += 1.0;
            let cur: bool = random();
            if cur && last {break;}
            last = cur;
        }
        sum += count;
    }
    sum / range as f64
}

pub fn send_count(range: i32) -> (Vec<JoinHandle<()>>, Receiver<f64>, i32) {
    let (tx, rx): (mpsc::Sender<f64>, mpsc::Receiver<f64>) = mpsc::channel();
    let mut handles = Vec::new();
    for _ in 0..range {
        let tx_clone = tx.clone();
        let h: JoinHandle<()> = thread::spawn(move || {
            let _ = tx_clone.send(counter());
        });
        handles.push(h);
    }
    (handles, rx, range)
}
pub fn receive_count(receivers : (Vec<JoinHandle<()>>, Receiver<f64>, i32)) -> f64 {
    let mut sum: f64 = 0.0;
    while let Ok(value) = receivers.1.recv() {
        sum += value;
    }
    sum / receivers.2 as f64
}
fn main() {
    let mut input_num_of_threads = String::new();
    println!("Enter number of threads you want:");
    io::stdin()
               .read_line(&mut input_num_of_threads)
               .expect("failed to read from stdin");
    let trimmed_input: &str = input_num_of_threads.trim();
    let mut range: i32 = 100;
    match trimmed_input.parse::<i32>() {
        Ok(i) => range = i,
        Err(..) => println!("Error! Not an int!"),
    };
    let start: Instant = Instant::now();
    println!("running {} threads, result: {}", range, receive_count(send_count(range)));
    let elapsed_time = start.elapsed().as_secs_f64();
    println!("Time taken: {}", elapsed_time);
}
