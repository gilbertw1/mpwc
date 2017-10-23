# mpwc

Super duper simple Master Password CLI client

### Installation

Build

    cargo build --release

Run

    ./target/release/mpwc

### Usage

```
mpwc 0.1.0
Bryan G. <bryan@bryan.sh>
Super duper simple Master Password CLI client. https://github.com/gilbertw1/mpwc
USAGE:
    mpwc [FLAGS] [OPTIONS] <SITE>
FLAGS:
    -h, --help       Prints help information
    -p, --prompt     If present prompts for master password, reads stdin otherwise
    -V, --version    Prints version information
OPTIONS:
    -c, --counter <COUNTER>    Site counter [default: 1]
    -n, --name <FULL_NAME>     Full name, defaults to environment variable $MPW_FULLNAME
    -t, --type <TYPE>          Password type (max, long, medium, short, basic, pin), defaults to environment variable
                               $MPW_SITETYPE [default: max]
ARGS:
    <SITE>    Site to generate password for
```

