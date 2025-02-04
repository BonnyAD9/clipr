# clipr
Command line utility to read from and write to system clipboard.

I was frustrated that xclip sometimes works and sometimes not, so i wrote this.

## Usage
```
Welcome in clipr by BonnyAD9
Version 0.1.0

Usage:
  clipr
    If stdin is terminal, paste. Otherwise pass.

  clipr <flag>
    Do what the flag says.

Flags:
  -h  -?  --help
    Show this help.

  -c  --copy
    Copy data from stdin to the system clipboard.

  -v  --paste
    Write data from system clipboard to stdout.

  -cv  --pass
    Copy data from stdin to the system clipboard. Also print the data to
    stdout.

  -vc  --rotate
    Print data from the clipboard, than copy data from stdin to the clipboard.
```
