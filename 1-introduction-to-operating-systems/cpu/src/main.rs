use std::{env, thread::sleep, time::Duration};

fn main() {
    if let Some(str) = env::args().nth(1) {
        loop {
            sleep(Duration::from_secs(1));
            println!("{}", str);
        }
    } else {
        eprintln!("usage: cpu <string>\n");
    }
}
