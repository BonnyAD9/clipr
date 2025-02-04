use std::io::{stderr, stdin, IsTerminal, StderrLock, StdinLock};

use termal::raw::{
    is_raw_mode_enabled, wait_for_stdin, IoProvider, ValueOrMut, WaitForIn,
};

#[derive(Debug, Copy, Clone, Default)]
pub struct StdieProvider();

impl WaitForIn for StdieProvider {
    fn wait_for_in(
        &self,
        timeout: std::time::Duration,
    ) -> termal::error::Result<bool> {
        wait_for_stdin(timeout)
    }
}

impl IoProvider for StdieProvider {
    type Out = StderrLock<'static>;

    type In = StdinLock<'static>;

    fn get_out(&mut self) -> termal::raw::ValueOrMut<'_, Self::Out> {
        ValueOrMut::Value(stderr().lock())
    }

    fn get_in(&mut self) -> termal::raw::ValueOrMut<'_, Self::In> {
        ValueOrMut::Value(stdin().lock())
    }

    fn is_out_terminal(&self) -> bool {
        stderr().is_terminal()
    }

    fn is_in_terminal(&self) -> bool {
        stdin().is_terminal()
    }

    fn is_out_raw(&self) -> bool {
        is_raw_mode_enabled()
    }
}
