mod cli;
mod copy;
mod err;
mod pass;
mod paste;
mod stdie_provider;

use std::process::ExitCode;

use cli::{help, Action, Args};
use pareg::Pareg;
use termal::eprintacln;

pub use self::{copy::*, err::*, pass::*, paste::*};

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
    }

    Ok(())
}
