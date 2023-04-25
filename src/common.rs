#![allow(dead_code)]

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

// コドゲサンプルに合わせたReader
pub fn get_input_line() -> String {
    use std::io;

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    // 邪魔な時はここをコメントアウト
    // eprint!("{}", input_line);

    input_line
}
