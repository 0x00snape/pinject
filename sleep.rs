use std::{thread, time::Duration};

fn main() {
    loop {
            println!("...buzz buzz buzz...");
            thread::sleep(Duration::new(1, 0));
    }

}

