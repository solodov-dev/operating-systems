use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    if let Some(str) = env::args().nth(1) {
        loop {
            sleep(Duration::from_secs(1));
            println!("{}\n", str);
        }
    } else {
        eprintln!("usage: cpu <string>\n");
    }
}
