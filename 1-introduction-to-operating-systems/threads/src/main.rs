use std::{
    env,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    thread,
};

fn main() {
    if let Some(loops) = env::args().nth(1).and_then(|a| a.parse().ok()) {
        let mut handles = vec![];
        let counter = Arc::new(AtomicU32::new(0));

        println!("Initial value: {:?}", counter);

        for _ in 0..2 {
            let c = counter.clone();
            let handle = thread::spawn(move || {
                for _ in 0..loops {
                    c.fetch_add(1, Ordering::Relaxed);
                }
            });

            handles.push(handle)
        }

        handles
            .into_iter()
            .for_each(|handle| handle.join().unwrap());

        println!("Final value: {:?}", counter);
    } else {
        eprintln!("usage: threads <value>\n");
    }
}
