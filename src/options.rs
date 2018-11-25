pub struct Options {
    pub short_output: bool,
    pub key_length: u16,
    pub format: Format,
}

pub enum Format {
    Ascii,
    AsciiBlanks,
    AsciiReduced,
    AlphaNumeric,
}

impl Options {
    pub fn from_args(args: clap::ArgMatches) -> Options {
        let options = Options {
            short_output: args.is_present("short"),
            // Deal with these unwraps
            // "length" will always have a value,
            // but it's not guaranteed to be a u16
            key_length: args.value_of("length").unwrap().parse().unwrap(),
            format: Format::Ascii,
        };

        options
    }
}
