use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

use super::options::Format;

pub fn generate_key(format: &Format, length: usize) -> String {
    match format {
        Format::AlphaNumeric => generate_alphanumeric_key(length),
        Format::Ascii => generate_ascii_key_without_blanks(length),
        _ => panic!(),
    }

    // let mut rng = rand::thread_rng();
    // let mut data: [u8; 3] = [0; 3];
    // rng.fill_bytes(&mut data);
    // println!("{:?}", data);
    // str::from_utf8(&data).unwrap().to_owned()
}

fn generate_ascii_key_without_blanks(length: usize) -> String {
    let mut rng = rand::thread_rng();
    iter::repeat(())
        .map(|()| rng.gen_range(33, 126) as u8 as char)
        .take(length)
        .collect()
}

fn generate_alphanumeric_key(length: usize) -> String {
    let mut rng = rand::thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(length)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_alphanumeric_with_length() {
        //given
        let format = Format::AlphaNumeric;
        let length = 9;

        //when
        let key = generate_key(&format, length);

        //then
        assert_eq!(key.len(), length as usize);
    }

    #[test]
    fn generates_alphanumeric_with_only_alphanumeric_chars() {
        //given
        let format = Format::AlphaNumeric;
        let length = 7;

        //when
        let key = generate_key(&format, length);

        //then
        assert!(key.chars().all(char::is_alphanumeric));
    }
}
