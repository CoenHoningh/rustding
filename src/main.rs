use argh::FromArgs;
use std::iter::repeat_with;

#[derive(FromArgs, PartialEq)]
/// Dit is een test
struct Ding {
    /// getal 1
    #[argh(positional)]
    first: usize,

    /// getal 2
    #[argh(positional)]
    second: usize,
}

fn main() {
    let Ding { first, second } = argh::from_env();
    let mut rng = fastrand::Rng::new();
    let mut subgen = move || repeat_with(|| rng.f64()).take(first).collect::<Vec<f64>>();
    let matrix: Vec<Vec<f64>> = repeat_with(|| subgen()).take(second).collect();
    eprintln!("{}", matrix.len());
}
