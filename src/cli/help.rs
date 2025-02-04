use termal::{gradient, printacln};

pub fn help() {
    let v = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
    let signature = gradient("BonnyAD9", (250, 50, 170), (180, 50, 240));

    printacln!(
        "Welcome in {'g i}clipr{'_} by {signature}{'_}
Version {v}

{'g}Usage:
  {'c}clipr{'_}
    If stdin is terminal, paste. Otherwise copy.

  {'c}clipr {'w}<{'y}flag{'w}>{'_}
    Do what the flag says.

{'g}Flags:
  {'y}-h  -?  --help{'_}
    Show this help.

  {'y}-c  --copy{'_}
    Copy data from stdin to the system clipboard. Stderr must be terminal.

  {'y}-v  --paste{'_}
    Write data from system clipboard to stdout. Stdin and stderr must be
    terminal.

  {'y}-cv  --pass{'_}
    Copy data from stdin to the system clipboard. Also print the data to
    stdout.
"
    )
}
