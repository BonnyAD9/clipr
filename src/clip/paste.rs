use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};

use arboard::Clipboard;
use termal::{
    codes,
    raw::{
        disable_raw_mode, enable_raw_mode,
        events::{Event, Status},
        Terminal,
    },
};

use crate::{std_provider::StdProvider, Error, Result};

pub fn paste() -> Result<()> {
    let d = paste_data()?;
    let mut out = stdout().lock();
    out.write_all(&d)?;
    out.flush()?;
    Ok(())
}

pub fn paste_data() -> Result<Vec<u8>> {
    paste_data_direct().or_else(|e| {
        paste_data_term().map_err(|e2| Error::Double(Box::new((e, e2))))
    })
}

pub fn paste_data_term() -> Result<Vec<u8>> {
    enable_raw_mode()?;
    let r = paste_data_term_inner();
    disable_raw_mode()?;
    r
}

fn paste_data_term_inner() -> Result<Vec<u8>> {
    let mut term = Terminal::<StdProvider>::default();

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

fn paste_data_direct() -> Result<Vec<u8>> {
    let mut cb = Clipboard::new()?;
    Ok(cb.get_text()?.into())
}
