#![allow(non_snake_case)]
mod common;
mod entity;
mod extvec;

#[allow(unused_imports)]
use rand::{thread_rng, Rng};
use std::time::SystemTime;

use common::*;
use entity::*;
use extvec::*;

fn main() {
    let mut _rng = thread_rng();

    let system_time = SystemTime::now();

    let _ = Hoge::Fuga;

    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
}

// eprintln!("");
