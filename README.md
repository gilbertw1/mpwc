# mpwc

A simple and flexible Master Password CLI client

### Installation

Build

    cargo build --release

Run

    ./target/release/mpwc

Example (prompt)

    ./mpwc -c 3 -t pin -n 'Arthur Dent' google.com

Example (stdin)

    echo 'supersecret' | ./mpwc -n 'Arthur Dent' facebook.com -i

### Usage

```
mpwc 0.1.0
Bryan G. <bryan@bryan.sh>
A simple and flexible Master Password CLI client. https://github.com/gilbertw1/mpwc
USAGE:
    mpwc [FLAGS] [OPTIONS] <SITE>
FLAGS:
    -h, --help       Prints help information
    -q, --quiet      If present only outputs password, excluding identicons
    -i, --stdin      If present reads password from stdin, reads from prompt otherwise
    -V, --version    Prints version information
OPTIONS:
    -c, --counter <COUNTER>    Site counter [default: 1]
    -n, --name <FULL_NAME>     Full name, falls back to $MPW_FULLNAME [env:MPW_FULLNAME]
    -t, --type <TYPE>          Password type (max, long, medium, short, basic, pin), falls back to $MPW_SITETYPE
                               [env:MPW_SITETYPE]  [default: max]
ARGS:
    <SITE>    Site to generate password for```

