# NAME

mpwc - A simple and flexible Master Password CLI client

# SYNOPSIS

mpwc [*flags*] [*options*] SITE

mpwc -h


# DESCRIPTION

Generate password for SITE using the master password algorithm.

mpwc is a simple command line client interface that generates site specific master passwords
and identicons. It's flexibility allows it to easily be used interactively or as part of a
script.

Project home page: https://github.com/gilbertw1/mpwc

# OPTIONS

-c, --counter *COUNTER*
: Site counter to use when generating site specific password
  [default: 1]

-n, --name *FULL_NAME*
: Full name to be used when generating site specific password
  [default: $MPW_FULLNAME]

-i, --stdin
: Read password from stdin otherwise password is read from interactive prompt

-q, --quiet
: Only outputs password, excluding identicons
 
-t, --type *SITE_TYPE*
: Type of password to generate. Possible Values: max, long, medium, short, basic, pin
  [default: $MPW_SITETYPE or max]
