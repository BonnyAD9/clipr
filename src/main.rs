mod cli;
mod clip;
mod err;
mod std_provider;

use std::process::ExitCode;

use cli::{help, Action, Args};
use clip::{copy, pass, paste, rotate};
use pareg::Pareg;
use termal::eprintacln;

pub use self::err::*;

fn main() -> ExitCode {
    match start() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintacln!("{'r}error: {'_}{e}");
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<()> {
    let args = Args::parse(Pareg::args())?;

    match args.action {
        Action::Help => help(),
        Action::Copy => copy()?,
        Action::Paste => paste()?,
        Action::Pass => pass()?,
        Action::Rotate => rotate()?,
    }

    Ok(())
}
