use std::io::{stderr, stdout, IsTerminal, StderrLock, StdoutLock, Write};

pub enum StdLock<'a> {
    Stdout(StdoutLock<'a>),
    Stderr(StderrLock<'a>),
}

impl StdLock<'static> {
    pub fn terminal_or_out() -> Self {
        if stdout().is_terminal() {
            Self::Stdout(stdout().lock())
        } else if stderr().is_terminal() {
            Self::Stderr(stderr().lock())
        } else {
            Self::Stdout(stdout().lock())
        }
    }
}

impl Write for StdLock<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            StdLock::Stdout(stdout_lock) => stdout_lock.write(buf),
            StdLock::Stderr(stderr_lock) => stderr_lock.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            StdLock::Stdout(stdout_lock) => stdout_lock.flush(),
            StdLock::Stderr(stderr_lock) => stderr_lock.flush(),
        }
    }
}
