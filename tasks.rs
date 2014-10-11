use std::comm::{channel, Sender, Receiver};

fn plus_one(sender: &Sender<int>, receiver: &Receiver<int>) {
    let mut value: int;
    loop {
        value = receiver.recv();
        sender.send(value + 1);
        if value == 0 { break; }
    }
}

fn main() {
    let (parent_sender, parent_receiver) = channel();
    let (child_sender, child_receiver) = channel();

    spawn(proc() {
        plus_one(&child_sender, &parent_receiver);
    });

    parent_sender.send(22);
    parent_sender.send(23);
    parent_sender.send(24);
    parent_sender.send(25);
    parent_sender.send(0);

    for _ in range(0i, 4) {
        println!("{:s}", child_receiver.recv().to_string());
    }
}
