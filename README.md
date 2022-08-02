# Demo of FFI binding for pcre2

## PCRE2

- [pcre2api specification](https://www.pcre.org/current/doc/html/pcre2api.html)
    - [pcre2demo](https://www.pcre.org/current/doc/html/pcre2demo.html)
    - [pcre2_compile](https://www.pcre.org/current/doc/html/pcre2_compile.html)

### Difference between functions with suffix `_8`、`_16`和`_32`

    By default, a library called libpcre2-8 is built, containing functions that
    take string arguments contained in arrays of bytes, interpreted either as
    single-byte characters, or UTF-8 strings. You can also build two other
    libraries, called libpcre2-16 and libpcre2-32, which process strings that
    are contained in arrays of 16-bit and 32-bit code units, respectively.
    These can be interpreted either as single-unit characters or UTF-16/UTF-32
    strings.

## How to play

1. run `make` in one terminal window
2. run `cargo run` in another terminal window
3. you will see regexp match result showing on the screen in step 1