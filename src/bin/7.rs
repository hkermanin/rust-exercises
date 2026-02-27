use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        println!("Spawn");
        thread::sleep(Duration::from_millis(3000));
    });

    println!("salam");
    handle.join().unwrap();
}
