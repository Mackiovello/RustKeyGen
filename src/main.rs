extern crate clap;
extern crate rand;

mod key_generator;
mod options;

use clap::{App, Arg};

use self::options::CommandOptions;

fn main() {
    let matches = App::new("KeyGen")
        .version("0.1")
        .about("Generates keys and passwords.")
        .arg(
            Arg::with_name("ascii")
                .short("a")
                .long("ascii")
                .help("Generates a key of ASCII characters, ranging from '!' to'~' (default)"))
        .arg(
            Arg::with_name("ascii_blank")
                .short("w")
                .long("ascii-blank")
                .help("Generates a key of ASCII characters, ranging from ' ' to'~'; same as --ascii, but includes blanks"))
        .arg(
            Arg::with_name("ascii_reduced")
                .short("r")
                .long("ascii-reduced")
                .help("Generates a key of reduced ASCII"))
        .arg(
            Arg::with_name("alphanum")
                .short("p")
                .long("alphanum")
                .help("Generates a key of alphanumeric characters"))
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .value_name("n")
                .default_value("8")
                .help("Generates a key of <n> length"))
        .get_matches();

    let options = CommandOptions::from_args(&matches);
    let key = key_generator::generate_key(&options.format, options.key_length);
    println!("{}", key);
}
