extern crate clap;
use clap::{Arg, App};
use harptool::run;

fn main() {
    let matches = App::new("harptool")
        .about("print harmonica note layouts")
        .arg(Arg::with_name("tuning")
             .short("t")
             .long("tuning")
             .value_name("TUNING")
             .help("select tuning"))
        .arg(Arg::with_name("key")
             .short("k")
             .long("key")
             .value_name("KEY")
             .help("select key"))
        .arg(Arg::with_name("sharps")
             .long("sharps")
             .help("use sharps"))
        .arg(Arg::with_name("flats")
             .long("flats")
             .help("use flats"))
        .get_matches();

    let tuning = matches.value_of("tuning").unwrap_or("richter");
    let key = matches.value_of("key").unwrap_or("C");
    let sharp = if matches.is_present("sharps") {
        Some(true)
    } else if matches.is_present("flats") {
        Some(false)
    } else {
        None
    };

    run(tuning, key, sharp);
}
