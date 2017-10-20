# mpwc
Super duper simple Master Password CLI client

```
mpwc 0.1.0
Bryan G. <bryan@bryan.sh>
Super duper simple Master Password CLI client. https://github.com/gilbertw1/mpwc
USAGE:
    mpwc [FLAGS] [OPTIONS] --site <SITE>
FLAGS:
    -h, --help       Prints help information
    -p, --prompt     If present prompts for master password, reads stdin otherwise
    -V, --version    Prints version information
OPTIONS:
    -c, --counter <COUNTER>    Site counter [default: 1]
    -n, --name <FULL_NAME>     Full name, defaults to environment variable $MPW_FULLNAME
    -s, --site <SITE>          Site to create password for
    -t, --type <TYPE>          Password type (max, long, medium, short, basic, pin), defaults to environment variable
                               $MPW_SITETYPE [default: max]
```

