extern crate clap;
use clap::{Arg, App};
use harmonica::run;

fn main() {
    let matches = App::new("harmonica tool")
	.author("Sebastian Thümmel")
	.about("print harmonica note layouts")
	.arg(Arg::with_name("tuning")
	     .short("t")
	     .long("tuning")
	     .value_name("TUNING")
	     .help("select the tuning"))
	.arg(Arg::with_name("key")
	     .short("k")
	     .long("key")
	     .value_name("KEY")
	     .help("select the key"))
	.arg(Arg::with_name("sharps")
	     .long("sharps")
	     .help("use sharps"))
	.arg(Arg::with_name("flats")
	     .long("flats")
	     .help("use flats"))
        .get_matches();

    let tuning = matches.value_of("tuning").unwrap_or("richter.txt");
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
