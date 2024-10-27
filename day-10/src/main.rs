//multithreading

use std::{sync::mpsc, thread::{self, sleep, spawn}, time::Duration};

fn main() {
   let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi from spawned thread {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("Hi from main thread {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    let x = 1;
    {
        let vect = vec![1,2,3];
        thread::spawn(move || {
            println!("{:?}",vect);
        });

    }
    print!("{x}");
    

   //Message passing
    let (tx, rx) = mpsc::channel();

    spawn(move || {
        tx.send(String::from("Hello world")).unwrap();
    });

    let recieved_message = rx.recv().unwrap();
    println!("{}",recieved_message);


    let (tx, rx) = mpsc::channel();

    spawn(move || {
        let vals = ["1", "2", "3", "go"];
        for val in vals {
            tx.send(val).unwrap();
        }
        sleep(Duration::from_millis(400));
    });


    for val in rx {
        println!("The value gotten is {}",val);
    }


}
