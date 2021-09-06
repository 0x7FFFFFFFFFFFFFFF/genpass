# genpass

This is a fork from [https://builds.sr.ht/~cyplo/genpass](https://builds.sr.ht/~cyplo/genpass). The following description is for this forked version.

![demo](https://github.com/0x7FFFFFFFFFFFFFFF/genpass/raw/master/genpass.gif "")


A simple, robust, fast, multiplatform commandline random password generator.

You can use it generate keys or passwords in scripts or use it as your primary desktop password generator.

Has extensive generative test suite, including tests against a [Rust port](https://crates.io/crates/zxcvbn) of Dropbox's password strength tester [`zxcvbn`](https://www.usenix.org/conference/usenixsecurity16/technical-sessions/presentation/wheeler)

## Update (9/6/2021)
* Added `-x` option to exclude characters in a user defined string.
* Added `-m` option to let the user to provide a string and at lease one of the character of it will be included in the generated password.

## Typical usage
```
genpass                  # use defaults, they're good
genpass 2048             # generate long password, can be used as a key
genpass --passphrase 128 # generate longer passphrase
genpass -dlu             # no special characters
```

## Copy to system clipboard

The generated password will be copied to the clipboard automatically.
```
genpass
```


## Installation
Download from the release page.

## Commandline options
```
$ genpass --help

genpass 1.1.0

USAGE:
    genpass.exe [FLAGS] [OPTIONS] [length]

FLAGS:
    -a, --all-character-types    Equals to "-dulse"
    -h, --help                   Prints help information
    -d, --include-digit          Include at least one digit
    -e, --include-extended       Include at least one character from Latin-1 Supplement, Latin Extended-A or Latin
                                 Extended-B
    -l, --include-lowercase      Include at least one lowercase letter
    -s, --include-special        Include at least one special (non-alphanumeric) character
    -u, --include-uppercase      Include at least one uppercase letter
    -p, --passphrase             Create a passphrase of (at least) the given length instead of a password.
        --version

OPTIONS:
    -x, --exclude <exclude-characters>     These characters will be excluded [default: empty string]
    -m, --include <must-include-one-of>    Must include one of these characters [default: empty string]

ARGS:
    <length>    The length of the password to generate [default: 50]
```

### A note on passphrases
* Passphrases are generated using [EFF's](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) "long" password list.
* Passphrases are at least `--length` characters long, not necessarily exactly that long.

## TODOs
* [support custom sets of characters](https://github.com/cyplo/genpass/issues/4)
* [add benchmarks](https://github.com/cyplo/genpass/issues/5)

## Contributing
All contributions welcome !
Ideally - start a discussion with an issue first before contributing a PR.
