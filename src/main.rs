extern crate clap;

use clap::{Arg, App};

fn main() {
    
    let matches = App::new("Binkget")
        .version("0.1.0")
        .author("Nathaniel Chappelle <nathaniel.chappelle@proton.me>")
        .about("wget clone written in Rust")
        .arg(Arg::with_name("URL")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("url to download"))
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    println!("URL: {}", url);
}
