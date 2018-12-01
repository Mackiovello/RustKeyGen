# key_gen

CLI generator for keys and password.

```txt
 ./key_gen --help
KeyGen 0.1
Generates keys and passwords.

USAGE:
    key_gen [FLAGS] [OPTIONS]

FLAGS:
    -p, --alphanum         Generates a key of alphanumeric characters
    -a, --ascii            Generates a key of ASCII characters, ranging from '!' to'~' (default)
    -w, --ascii-blank      Generates a key of ASCII characters, ranging from ' ' to'~'; same as --ascii, but includes
                           blanks
    -r, --ascii-reduced    Generates a key of reduced ASCII
    -h, --help             Prints help information
    -V, --version          Prints version information

OPTIONS:
    -l, --length <n>    Generates a key of <n> length [default: 8]
```

## Build and test

Build and testing requires Rust 2018 edition. With that in place, you can run `cargo build` to build and `cargo test` to test.