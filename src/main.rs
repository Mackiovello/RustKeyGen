extern crate clap;

use clap::{App, Arg};

fn main() {
    App::new("KeyGen")
        .version("0.1")
        .about("Generates keys and passwords.")
        .arg(Arg::with_name("directory").short("a").long("ascii").help(
            "Generates a key of ASCII characters, ranging from '!' to'~' (default)",
        ))
        .arg(
            Arg::with_name("ascii_blank")
                .short("w")
                .long("ascii-blank")
                .help(
                    "Generates a key of ASCII characters, ranging from ' ' to'~'; same as --ascii, but includes blanks",
                ),
        )
        .arg(
            Arg::with_name("ascii_reduced")
                .short("r")
                .long("ascii-reduced")
                .help("Generates a key of reduced ASCII"),
        )
        .arg(
            Arg::with_name("alphanum")
                .short("p")
                .long("alphanum")
                .help("Generates a key of alphanumeric characters"),
        )
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .value_name("n")
                .help("Generates a key of <n> bytes length"),
        )
        .arg(
            Arg::with_name("short")
                .short("s")
                .long("short")
                .help("Shows only the key"),
        )
        .get_matches();
}
