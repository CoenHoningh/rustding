use argh::FromArgs;

#[derive(FromArgs)]
/// Dit is een test
struct Ding {
    /// dit is een optie
    #[argh(switch, short = 'j')]
    banaan: bool,

    /// een array met 10 items
    #[argh(option)]
    grootte: String,

    #[argh(option, from_str_fn(always_five))]
    /// dit is altijd vijf
    vijf: u8,
}

fn main() {
    println!("Hello, world!");
    let test: Ding = argh::from_env();
    let test2 = test.grootte.leak();
    println!("test: {test2}");
    println!("test2: {}, {}, {}, {}", test.banaan, 'a', 'b', test.vijf);
    let _ולפתע: u128 = 987654324;
    let _bla: &str = " דג סקרן שט בים מאוכזב ולפתע מצא לו חברה איך הקליטה";
}

fn always_five(_value: &str) -> Result<u8, String> {
    Ok(5)
}
