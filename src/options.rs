#[derive(Debug)]
pub struct CommandOptions {
    pub key_length: usize,
    pub format: Format,
}

#[derive(Debug)]
pub enum Format {
    Ascii,
    AsciiBlanks,
    AlphaNumeric,
}

impl CommandOptions {
    pub fn from_args(args: &clap::ArgMatches) -> Result<CommandOptions, String> {
        let length = match args.value_of("length").unwrap().parse() {
            Ok(v) => v,
            Err(_) => return Err("The length has to be a number.".to_owned()),
        };

        if length == 0 {
            return Err("The length has to be larger than 0.".to_owned());
        }

        Ok(CommandOptions {
            // Deal with these unwraps
            // "length" will always have a value,
            // but it's not guaranteed to be a u16.
            key_length: length,
            format: CommandOptions::format_from_args(&args),
        })
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
        } else if args.is_present("alphanum") {
            Format::AlphaNumeric
        } else {
            // The default value
            Format::Ascii
        }
    }
}
