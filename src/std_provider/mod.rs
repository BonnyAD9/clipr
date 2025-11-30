use std::io::{stderr, stdin, IsTerminal, StdinLock};

use termal::raw::{
    is_raw_mode_enabled, wait_for_stdin, IoProvider, ValueOrMut, WaitForIn,
};

mod std_lock;

pub use self::std_lock::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct StdProvider();

impl WaitForIn for StdProvider {
    fn wait_for_in(
        &self,
        timeout: std::time::Duration,
    ) -> termal::Result<bool> {
        wait_for_stdin(timeout)
    }
}

impl IoProvider for StdProvider {
    type Out = StdLock<'static>;

    type In = StdinLock<'static>;

    fn get_out(&mut self) -> termal::raw::ValueOrMut<'_, Self::Out> {
        ValueOrMut::Value(StdLock::terminal_or_out())
    }

    fn get_in(&mut self) -> termal::raw::ValueOrMut<'_, Self::In> {
        ValueOrMut::Value(stdin().lock())
    }

    fn is_out_terminal(&self) -> bool {
        stdin().is_terminal() || stderr().is_terminal()
    }

    fn is_in_terminal(&self) -> bool {
        stdin().is_terminal()
    }

    fn is_out_raw(&self) -> bool {
        is_raw_mode_enabled()
    }
}
