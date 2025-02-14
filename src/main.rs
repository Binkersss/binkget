extern crate clap;
use binkget::download;

use clap::{Arg, App};
use tokio::runtime::Builder;

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

    let quiet_mode = false;

    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(e) = rt.block_on(download(url, quiet_mode)) {
        eprintln!("Download failed: {}", e);
    }

}
