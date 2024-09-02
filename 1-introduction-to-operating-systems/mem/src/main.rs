use std::{process::id as pid, ptr::addr_of, thread::sleep, time::Duration};

fn main() {
    let mut p = Box::new(0);
    println!("({}) address of p: {:?}", pid(), addr_of!(p));

    loop {
        sleep(Duration::from_secs(1));
        *p += 1;
        println!("({}) p: {}", pid(), p);
    }
}
