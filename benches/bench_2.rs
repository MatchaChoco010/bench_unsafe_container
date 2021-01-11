#![feature(test)]
extern crate test;

use bench_unsafe_container::Container;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

struct World {
    field: f64,
}

#[bench]
fn mutex_read(b: &mut test::Bencher) {
    b.iter(|| {
        let mut handles = vec![];

        let x = Arc::new(Mutex::new(World { field: 0.0 }));

        for _ in 0..4 {
            let x = Arc::clone(&x);
            let handle = thread::spawn(move || {
                for _ in 0..10000 {
                    let x = x.lock().unwrap();
                    let y = x.field;
                }
            });
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }
    });
}

#[bench]
fn rwlock_read(b: &mut test::Bencher) {
    b.iter(|| {
        let mut handles = vec![];

        let x = Arc::new(RwLock::new(World { field: 0.0 }));

        for _ in 0..4 {
            let x = Arc::clone(&x);
            let handle = thread::spawn(move || {
                for _ in 0..10000 {
                    let x = x.read().unwrap();
                    let y = x.field;
                }
            });
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }
    });
}

#[bench]
fn container_read(b: &mut test::Bencher) {
    b.iter(|| {
        let mut handles = vec![];

        let x = Arc::new(Container::new(World { field: 0.0 }));

        for _ in 0..4 {
            let x = Arc::clone(&x);
            let handle = thread::spawn(move || {
                for _ in 0..10000 {
                    let x = unsafe { x.read() };
                    let y = x.field;
                }
            });
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }
    });
}
