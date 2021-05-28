# owner-edit

A small command line utility for editing environment ownership configuration files.

## help

```shell
$ owner-edit --help
owner-edit 0.2.0

USAGE:
    owner-edit.exe [FLAGS] <path> <SUBCOMMAND>

FLAGS:
    -f, --force
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <path>

SUBCOMMANDS:
    add
    help    Prints this message or the help of the given subcommand(s)
    rm

$ owner-edit add --help
owner-edit.exe-add 0.2.0

USAGE:
    owner-edit.exe <path> add [OPTIONS] [--] [users]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --environment <environments>...    
    -g, --group <groups>...                

ARGS:
    <users>...    

$ owner-edit rm --help
owner-edit.exe-rm 0.2.0

USAGE:
    owner-edit.exe <path> rm [OPTIONS] [--] [users]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --environment <environments>...    
    -g, --group <groups>...                

ARGS:
    <users>...    
```
