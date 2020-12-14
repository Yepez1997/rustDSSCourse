
use clap::{App, Arg, SubCommand};
use std::process;


fn main() {
    println!("rust distributed sytems ....");
    let matches = App::new("KVstore")
        .version("1.0")
        .author("Aureliano Y. <yepez4845@gmail.com>")
        .about("Distributed Key Value Store")
        .arg(
            Arg::with_name("get")
                .takes_value(true)
                .index(1)
                .help("get key from store ex. get key"),
        )
        .arg(
            Arg::with_name("set")
                .help("set key to store ex. set key value")
        )
        .arg(
            Arg::with_name("remove")
                .help("remove key from store ex.remove key")
        )
        .arg(
            Arg::with_name("version")
                .short("V"),
        )
        .get_matches();


    if let Some(o) = matches.value_of("get") {
        println!("Value for output: {}", o);
    }

    if let Some(o) = matches.value_of("V") {
        println!("here");
        let version = env!("CARGO_PKG_VERSION");
        println!("CARGO VERSION : {}", version);
    }

    std::process::exit(1);
}



