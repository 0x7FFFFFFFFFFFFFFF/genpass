use crate::alphabet::Alphabets;
use crate::alphabet::generate_alphabet;

const MINIMUM_PASSWORD_LENGTH: usize = 5;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Source {
    Alphabets(Alphabets),
    Words,
}

#[derive(Clone, Debug)]
pub struct GenerationOptions {
    pub length: usize,
    pub source: Source,
    pub exclude_chars: String,
}

pub fn generate_password<Rng: rand::Rng>(options: GenerationOptions, rng: &mut Rng) -> String {
    let length = options.length;
    if length < MINIMUM_PASSWORD_LENGTH {
        panic!(
            "Given the generation options, the length of {} is too short",
            length
        );
    }

    match options.source {
        Source::Alphabets(alphabets) => {
            let alphabet = generate_alphabet(alphabets);
            return generate_password_from_alphabet(options.length, rng, &alphabet, &options.exclude_chars);
        }
        Source::Words => {
            let mut passphrase = String::with_capacity(length);
            loop {
                passphrase.push_str(eff_wordlist::large::random_word());
                if passphrase.chars().count() >= length {
                    return passphrase;
                }
                passphrase.push(' ');
            }
        }
    }
}

fn generate_password_from_alphabet<Rng: rand::Rng>(
    length: usize,
    rng: &mut Rng,
    alphabet: &[char],
    excluded_chars: &str,
) -> String {
    let mut password = String::with_capacity(length);
    let mut size = 0;
    while size < length {
        let next_character_index = rng.next_u32() as usize % alphabet.len();
        let next_character = alphabet[next_character_index];
        if excluded_chars.contains(next_character) {
            continue;
        }
        password.push(next_character);
        size += 1;
    }
    password
}

