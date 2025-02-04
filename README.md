# clipr
Command line utility to read from and write to system clipboard.

I was frustrated that xclip sometimes works and sometimes not, so i wrote this.

Current implementation works by asking the terminal. So in all cases, stderr
must be terminal and when pasting (`-v`) stdin also must be terminal.

## Usage
```
Welcome in clipr by BonnyAD9
Version 0.1.0

Usage:
  clipr
    If stdin is terminal, paste. Otherwise copy.

  clipr <flag>
    Do what the flag says.

Flags:
  -h  -?  --help
    Show this help.

  -c  --copy
    Copy data from stdin to the system clipboard. Stderr must be terminal.

  -v  --paste
    Write data from system clipboard to stdout. Stdin and stderr must be
    terminal.

  -cv  --pass
    Copy data from stdin to the system clipboard. Also print the data to
    stdout.
```
