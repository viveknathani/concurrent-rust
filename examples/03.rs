use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    let second_tx = tx.clone();

    thread::spawn(move || {
        let val = String::from("hi from first producer");
        println!("sender sent: {:?}", val);
        tx.send(val).unwrap();
    });

    thread::spawn(move || {
        let val = String::from("hi from second producer");
        println!("sender sent: {:?}", val);
        second_tx.send(val).unwrap();
    });

    for msg in rx {
        println!("got: {:?}", msg);
    }
}
