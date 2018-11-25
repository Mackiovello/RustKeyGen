#[derive(Debug)]
pub struct CommandOptions {
    pub short_output: bool,
    pub key_length: u16,
    pub format: Format,
}

#[derive(Debug)]
pub enum Format {
    Ascii,
    AsciiBlanks,
    AsciiReduced,
    AlphaNumeric,
}

impl CommandOptions {
    pub fn from_args(args: clap::ArgMatches) -> CommandOptions {
        CommandOptions {
            short_output: args.is_present("short"),

            // Deal with these unwraps
            // "length" will always have a value,
            // but it's not guaranteed to be a u16.
            key_length: args.value_of("length").unwrap().parse().unwrap(),
            format: CommandOptions::format_from_args(&args),
        }
    }

    fn format_from_args(args: &clap::ArgMatches) -> Format {
        // I don't think this is the best way to
        // do this, but this is how it's done in
        // the original KeyGen. I'd rather
        // have format as an option that takes
        // a value since you can only set one
        // at once.
        if args.is_present("ascii") {
            Format::Ascii
        } else if args.is_present("ascii_blank") {
            Format::AsciiBlanks
        } else if args.is_present("ascii_reduced") {
            Format::AsciiReduced
        } else if args.is_present("alphanum") {
            Format::AlphaNumeric
        } else {
            // The default value
            Format::Ascii
        }
    }
}
