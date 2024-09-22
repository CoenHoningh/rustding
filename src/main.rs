use argh::FromArgs;
use std::{
    env::current_dir,
    fs::File,
    io::{LineWriter, Write},
    iter::repeat_with,
    os::windows::fs::MetadataExt,
};

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
    let mut rn_f64 = || rng.char(..);
    let subgen = || repeat_with(&mut rn_f64).take(first).collect::<String>();
    let matrix = repeat_with(subgen).take(second).collect::<Vec<String>>();
    if let Ok(mut _a) = dbg!(current_dir()) {
        _a.push("target/outp.txt");
        _a.set_extension("txt");
        if let Ok(bestand) = File::create(_a.as_path()) {
            let mut bestand = LineWriter::new(bestand);
            bestand
                .write_all(matrix.join("\n").as_bytes())
                .unwrap_or(());
            bestand.flush().unwrap_or(());
        };
        if let Ok(meta) = _a.metadata() {
            dbg!(meta.file_size());
        };
    };
}
