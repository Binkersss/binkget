extern crate clap;
use binkget::parallel_download;

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
        .arg(Arg::with_name("FILENAME")
                .required(true)
                .takes_value(true)
                .index(2)
                .help("target file name"))
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    println!("URL: {}", url);
    let fname = matches.value_of("FILENAME").unwrap();
    println!("File name: {}", fname);

    let num_connections = 4;
    let quiet_mode = false;
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(e) = rt.block_on(parallel_download(url, fname, num_connections, quiet_mode)) {
        eprintln!("Download failed: {}", e);
    }

}
