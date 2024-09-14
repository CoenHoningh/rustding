use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Dit is een test
struct Ding {
    /// dit is een optie
    #[argh(switch, short = 'j')]
    banaan: bool,

    /// een array met 10 items
    #[argh(option)]
    grootte: String,
}

fn main() {
    println!("Hello, world!");
    let test: Ding = argh::from_env();
    println!("test: {test:?}");
    println!("test2: {}, {}", test.banaan, test.grootte);
}
