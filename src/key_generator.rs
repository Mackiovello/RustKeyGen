use rand::distributions::Alphanumeric;
use rand::Rng;

use super::options::{CommandOptions, Format};

pub fn generate_key(options: &CommandOptions) -> String {
    match options.format {
        Format::AlphaNumeric => generate_alphanumeric_key(options.key_length),
        Format::Ascii => generate_ascii_key_from_range(options.key_length, 33, 126),
        Format::AsciiBlanks => generate_ascii_key_from_range(options.key_length, 32, 126),
    }
}

fn generate_ascii_key_from_range(length: usize, start: u16, end: u16) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.gen_range(start, end) as u8 as char)
        .collect()
}

fn generate_alphanumeric_key(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.sample(Alphanumeric)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_alphanumeric_with_length() {
        //given
        let options = CommandOptions {
            format: Format::AlphaNumeric,
            key_length: 9,
        };

        //when
        let key = generate_key(&options);

        //then
        assert_eq!(key.len(), options.key_length);
    }

    #[test]
    fn generates_alphanumeric_with_only_alphanumeric_chars() {
        //given
        let options = CommandOptions {
            format: Format::AlphaNumeric,
            key_length: 7,
        };

        //when
        let key = generate_key(&options);

        //then
        assert!(key.chars().all(char::is_alphanumeric));
    }
}
