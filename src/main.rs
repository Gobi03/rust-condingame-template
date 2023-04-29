#![allow(non_snake_case)]
mod common;
mod entity;
mod extvec;
mod reader;

#[allow(unused_imports)]
use rand::prelude::SliceRandom;
#[allow(unused_imports)]
use rand::{thread_rng, Rng};
use std::time::SystemTime;

use common::*;
use entity::*;
use reader::*;

fn main() {
    let mut _rng = thread_rng();

    loop {
        let system_time = SystemTime::now();

        let _ = Hoge::Fuga;

        // input sample
        let input_line = get_input_line();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let (w, h) = readWH(inputs);

        eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
    }
}

// eprintln!("");
