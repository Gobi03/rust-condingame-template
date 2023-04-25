// Reader
#[macro_export]
macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

// sample
pub fn readWH(inputs: Vec<&str>) -> (usize, usize) {
    let width = parse_input!(inputs[0], usize);
    let height = parse_input!(inputs[1], usize);

    (width, height)
}
