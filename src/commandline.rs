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
    default_value = "",
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

pub fn generation_options_for_commandline_options(
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
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub fn default_commandline_options() -> CommandlineOptions {
        let empty_args: [String; 0] = [];
        CommandlineOptions::from_iter(&empty_args)
    }
}

#[cfg(test)]
mod must {
    use crate::generator::Source::Words;

    use super::*;
    use super::test::*;

    #[test]
    fn support_lowercase_letters() {
        let mut commandline_options = default_commandline_options();
        commandline_options.include_lowercase = true;

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert_is_subalphabet(generation_options.source, Alphabets::LOWERCASE);
    }

    #[test]
    fn support_uppercase_letters() {
        let mut commandline_options = default_commandline_options();
        commandline_options.include_uppercase = true;

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert_is_subalphabet(generation_options.source, Alphabets::UPPERCASE);
    }

    #[test]
    fn support_digits() {
        let mut commandline_options = default_commandline_options();
        commandline_options.include_digit = true;

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert_is_subalphabet(generation_options.source, Alphabets::DIGIT);
    }

    #[test]
    fn support_special_chars() {
        let mut commandline_options = default_commandline_options();
        commandline_options.include_special = true;

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert_is_subalphabet(generation_options.source, Alphabets::SPECIAL);
    }

    #[test]
    fn support_passphrases() {
        let mut commandline_options = default_commandline_options();
        commandline_options.passphrase = true;

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert_eq!(generation_options.source, Words);
    }

    #[test]
    fn default_to_all_alphabets_with_default_commandline_flags() {
        let commandline_options = default_commandline_options();

        let generation_options = generation_options_for_commandline_options(commandline_options);

        assert_is_subalphabet(generation_options.source, Alphabets::all());
    }

    fn assert_is_subalphabet(actual: Source, expected: Alphabets) {
        match actual {
            Source::Alphabets(alphabets) => {
                assert_eq!(alphabets, expected);
            }
            Source::Words => panic!("Not an alphabet"),
        }
    }
}
