use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};

use termal::{
    codes,
    raw::{
        disable_raw_mode, enable_raw_mode,
        events::{Event, Status},
        Terminal,
    },
};

use crate::{stdie_provider::StdieProvider, Error, Result};

pub fn paste() -> Result<()> {
    let d = paste_data()?;
    let mut out = stdout().lock();
    out.write_all(&d)?;
    out.flush()?;
    Ok(())
}

pub fn paste_data() -> Result<Vec<u8>> {
    enable_raw_mode()?;
    let r = paste_data_inner();
    disable_raw_mode()?;
    r
}

fn paste_data_inner() -> Result<Vec<u8>> {
    let mut term = Terminal::<StdieProvider>::default();

    if !term.is_in_terminal() || !term.is_out_terminal() {
        return Err(Error::NoClipboardProvider);
    }

    term.write_all(codes::REQUEST_SELECTION.as_bytes())?;
    term.flush()?;

    let mut now = Instant::now();
    let end = now + Duration::from_secs(1);
    while now < end {
        if let Some(Event::Status(Status::SelectionData(d))) =
            term.read_timeout(end - now)?
        {
            return Ok(d);
        }
        now = Instant::now();
    }

    Err(Error::Timeout)
}
