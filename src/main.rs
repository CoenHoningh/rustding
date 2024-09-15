use std::{borrow::Borrow, collections::LinkedList};

use argh::FromArgs;

#[derive(FromArgs, PartialEq)]
/// Dit is een test
struct Ding {
    /// getal 1
    #[argh(positional)]
    first: i32,

    /// getal 2
    #[argh(positional)]
    second: i32,

    /// dit is een optie
    #[argh(switch, short = 'j')]
    banaan: bool,
}

fn main() {
    println!("Hello, world!");
    let test: Ding = argh::from_env();

    if test.banaan {
        println!("{}", test.first * test.second);
    } else {
        let division: f64 = Into::<f64>::into(test.first) / Into::<f64>::into(test.second);
        println!("{division:.20}");
    }
    let lijst = LinkedList::<i64>::from_iter(0..100);
    let lijstbox = Box::new(lijst.borrow());
    let lijstaddr = &lijst as *const LinkedList<i64>;
    let lijstboxaddr = &lijstbox as *const Box<&LinkedList<i64>>;
    println!("{:p}", lijstaddr);
    println!("{:p}", &lijstbox);
    println!("{:#?}", lijstboxaddr);
    println!("{}", lijstboxaddr as usize - lijstaddr as usize);
    let _ולפתע: u128 = 987654324;
    let _bla: &str = " דג סקרן שט בים מאוכזב ולפתע מצא לו חברה איך הקליטה";
}
