
use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
    println!("rust distributed sytems ....");
    let matches = App::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .help("Sets an optional output file")
                .index(1),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .multiple(true)
                .help("Turn debugging information on"),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("does testing things")
                .arg(Arg::with_name("list").short("l").help("lists test values")),
        )
        .get_matches();


    if let Some(o) = matches.value_of("output") {
        println!("Value for output: {}", o);
    }

    std::process::exit(1);
}



