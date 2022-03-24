# Hurust

Hurust is command line tool which act as wrapper of hugo new command.

## Features
* Automatically create new branch and checkout with content name.
* Automatically create new content file.

## Usage
```sh
hurust <subcommands> [options]
```

### Subcommands

```sh
USAGE:
    hurust <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help            Print this message or the help of the given subcommand(s)
    new -n --new    create new post
```

### Subcommands options
```sh
USAGE:
    hurust {new|--new|-n} --title <title>

OPTIONS:
    -h, --help             Print help information
    -t, --title <title>    input content title
```

