use std::{
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

fn main() {
    let mut myObj = MyStrct { etwas: 5 };
    let data = Arc::new(Mutex::new(myObj));
    let data2 = data.clone();

    let handle = spawn(move || loop {
        // move: take what you need and keep it
        println!("notice me");

        {
            let mut myObj = data.lock().unwrap();

            myObj.etwas += 1;
        }

        sleep(Duration::from_secs(5));
    });

    let handle2 = spawn(move || loop {
        println!("notice me 2");

        {
            let mut myObj = data2.lock().unwrap();

            myObj.etwas += 1;
        }

        sleep(Duration::from_secs(3));
    });

    handle.join().unwrap(); // dont memorize
    handle2.join().unwrap(); // dont memorize
}

struct MyStrct {
    etwas: i32,
}
