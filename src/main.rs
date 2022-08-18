extern crate clap;
use clap::{App, Arg};
use harptool::*;

fn is_valid_position(val: String) -> Result<(), String> {
    if let Ok(res) = val.parse::<usize>() {
        if (1..=12).contains(&res) {
            Ok(())
        } else {
            Err(String::from("must be between 1 and 12"))
        }
    } else {
        Err(String::from("must be integer"))
    }
}

fn main() {
    let matches = App::new("harptool")
        .about("print harmonica note layouts")
        .arg(
            Arg::with_name("tuning")
                .short("t")
                .long("tuning")
                .value_name("TUNING")
                .help("select tuning"),
        )
        .arg(
            Arg::with_name("key")
                .short("k")
                .long("key")
                .value_name("KEY")
                .help("select key"),
        )
        .arg(Arg::with_name("sharps").long("sharps").help("use sharps"))
        .arg(Arg::with_name("flats").long("flats").help("use flats"))
        .arg(
            Arg::with_name("degrees")
                .long("degrees")
                .short("d")
                .help("print scale degrees"),
        )
        .arg(
            Arg::with_name("position")
                .long("position")
                .short("p")
                .value_name("POSITION")
                .help("set position")
                .validator(is_valid_position),
        )
        .arg(
            Arg::with_name("scale")
                .long("scale")
                .short("s")
                .value_name("SCALE")
                .help("highlight notes of a scale"),
        )
        .arg(
            Arg::with_name("list tunings")
                .long("list-tunings")
                .short("l")
                .help("list available tunings"),
        )
        .get_matches();

    if matches.is_present("list tunings") {
        list_tunings();
        return;
    }
    let tuning = matches.value_of("tuning").unwrap_or("richter");
    let key = matches.value_of("key").unwrap_or("C");
    let sharp = if matches.is_present("sharps") {
        Some(true)
    } else if matches.is_present("flats") {
        Some(false)
    } else {
        None
    };
    let scale = matches.value_of("scale");
    let position = matches
        .value_of("position")
        .unwrap_or("1")
        .parse::<usize>()
        .unwrap();
    let setup = Setup { scale, position };

    if matches.is_present("degrees") {
        run_degrees(tuning, setup);
    } else {
        run(tuning, key, sharp, setup);
    }
}
