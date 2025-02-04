use std::io::{stdin, IsTerminal};

use pareg::Pareg;

use crate::Result;

use super::Action;

pub struct Args {
    pub action: Action,
}

impl Args {
    pub fn parse(mut args: Pareg) -> Result<Self> {
        let mut action = None;

        while let Some(arg) = args.next() {
            match arg {
                "-h" | "-?" | "--help" => action = Some(Action::Help),
                "-c" | "--copy" => action = Some(Action::Copy),
                "-v" | "--paste" => action = Some(Action::Paste),
                "-cv" | "--pass" => action = Some(Action::Pass),
                _ => return args.err_unknown_argument().err()?,
            }
        }

        Ok(Self {
            action: action
                .or_else(|| stdin().is_terminal().then_some(Action::Paste))
                .unwrap_or(Action::Copy),
        })
    }
}
