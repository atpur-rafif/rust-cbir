#![allow(unused)]

use std::marker::Send;
use std::sync::mpsc::{self, Sender};
use std::thread;

pub fn thread_pool<T: Send + 'static, U: Send + 'static>(
    mut values: Vec<T>,
    functor: fn(T) -> U,
) -> Vec<U> {
    let max_thread: usize = thread::available_parallelism().unwrap().into();
    let size = values.len();
    let (tx, rx) = mpsc::channel::<U>();

    fn inner_spawn<T: Send + 'static, U: Send + 'static>(
        value: T,
        functor: fn(T) -> U,
        transmitter: Sender<U>,
    ) {
        thread::spawn(move || {
            transmitter.send(functor(value)).unwrap();
        });
    }

    let mut res = Vec::new() as Vec<U>;

    let mut started = 0;
    while started < max_thread && started < size {
        inner_spawn(values.pop().unwrap(), functor, tx.clone());
        started += 1;
    }

    let mut finished = 0;
    while finished < size {
        match rx.try_recv() {
            Ok(v) => {
                res.push(v);
                match values.pop() {
                    Some(v) => inner_spawn(v, functor, tx.clone()),
                    None => {}
                }
                finished += 1;
            }
            Err(e) => {}
        }
    }

    res
}
