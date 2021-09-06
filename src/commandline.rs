use structopt::StructOpt;

use crate::alphabet::Alphabets;
use crate::generator::{GenerationOptions, Source};

#[derive(Clone, StructOpt, Debug)]
#[structopt(name = "genpass")]
pub struct CommandlineOptions {
    #[structopt(long = "version")]
    pub print_version: bool,

    #[structopt(
    long = "length",
    help = "The length of the password to generate",
    default_value = "50",
    index = 1
    )]
    pub length: usize,

    #[structopt(
    short = "p",
    long = "passphrase",
    help = "Create a passphrase of (at least) the given length instead of a password."
    )]
    passphrase: bool,

    #[structopt(
    short = "l",
    long = "include-lowercase",
    help = "Include at least one lowercase letter",
    conflicts_with = "passphrase"
    )]
    include_lowercase: bool,

    #[structopt(
    short = "u",
    long = "include-uppercase",
    help = "Include at least one uppercase letter",
    conflicts_with = "passphrase"
    )]
    include_uppercase: bool,

    #[structopt(
    short = "d",
    long = "include-digit",
    help = "Include at least one digit",
    conflicts_with = "passphrase"
    )]
    include_digit: bool,

    #[structopt(
    short = "s",
    long = "include-special",
    help = "Include at least one special (non-alphanumeric) character",
    conflicts_with = "passphrase"
    )]
    include_special: bool,

    #[structopt(
    short = "e",
    long = "include-extended",
    help = "Include at least one character from Latin-1 Supplement, Latin Extended-A or Latin Extended-B",
    conflicts_with = "passphrase"
    )]
    include_extended: bool,

    #[structopt(
    short = "x",
    long = "exclude",
    default_value = "empty string",
    help = "These characters will be excluded",
    )]
    exclude_characters: String,

    #[structopt(
    short = "a",
    long = "all-character-types",
    help = r#"Equals to "-dulse""#,
    conflicts_with = "passphrase"
    )]
    all_character_types: bool,
}

pub fn get_password_generation_options(
    args: CommandlineOptions,
) -> GenerationOptions {
    let source = if args.passphrase {
        Source::Words
    } else {
        let mut alphabets = Alphabets::empty();
        if args.include_lowercase {
            alphabets |= Alphabets::LOWERCASE;
        }
        if args.include_uppercase {
            alphabets |= Alphabets::UPPERCASE;
        }
        if args.include_digit {
            alphabets |= Alphabets::DIGIT;
        }
        if args.include_special {
            alphabets |= Alphabets::SPECIAL;
        }
        if args.include_extended {
            alphabets |= Alphabets::EXTENDED;
        }
        if args.all_character_types {
            alphabets |= Alphabets::LOWERCASE;
            alphabets |= Alphabets::UPPERCASE;
            alphabets |= Alphabets::DIGIT;
            alphabets |= Alphabets::SPECIAL;
            alphabets |= Alphabets::EXTENDED;
        }
        if alphabets.is_empty() {
            alphabets = Alphabets::all() - Alphabets::EXTENDED;
        }
        Source::Alphabets(alphabets)
    };
    GenerationOptions {
        length: args.length,
        source,
        exclude_chars: args.exclude_characters,
    }
}

