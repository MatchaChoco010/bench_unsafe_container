#![feature(test)]
extern crate test;

use bench_unsafe_container::Container;
use std::sync::{Mutex, RwLock};

struct World {
    field: f64,
}

#[bench]
fn mutex_read(b: &mut test::Bencher) {
    b.iter(|| {
        let x = Mutex::new(World { field: 0.0 });
        for _ in 0..10000 {
            let x = x.lock().unwrap();
            let y = x.field;
        }
    });
}

#[bench]
fn rwlock_read(b: &mut test::Bencher) {
    b.iter(|| {
        let x = RwLock::new(World { field: 0.0 });
        for _ in 0..10000 {
            let x = x.read().unwrap();
            let y = x.field;
        }
    });
}

#[bench]
fn container_read(b: &mut test::Bencher) {
    b.iter(|| {
        let x = Container::new(World { field: 0.0 });
        for _ in 0..10000 {
            let x = unsafe { x.read() };
            let y = x.field;
        }
    });
}

#[bench]
fn mutex_write(b: &mut test::Bencher) {
    b.iter(|| {
        let x = Mutex::new(World { field: 0.0 });
        for i in 0..10000 {
            let mut x = x.lock().unwrap();
            x.field = i as f64;
        }
    });
}

#[bench]
fn rwlock_write(b: &mut test::Bencher) {
    b.iter(|| {
        let x = RwLock::new(World { field: 0.0 });
        for i in 0..10000 {
            let mut x = x.write().unwrap();
            x.field = i as f64;
        }
    });
}

#[bench]
fn container_write(b: &mut test::Bencher) {
    b.iter(|| {
        let x = Container::new(World { field: 0.0 });
        for i in 0..10000 {
            let mut x = unsafe { x.write() };
            x.field = i as f64;
        }
    });
}
