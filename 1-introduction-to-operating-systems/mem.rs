use std::process;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut p = Box::new(0);
    println!("({}) address of p: {:?}", process::id(), ptr::addr_of!(p));

    loop {
        sleep(Duration::from_secs(1));
        *p += 1;
        println!("({}) p: {}", process::id(), p);
    }
}
