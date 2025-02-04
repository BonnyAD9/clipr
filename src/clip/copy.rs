use std::io::{stdin, Read, Write};

use arboard::Clipboard;
use termal::{codes, raw::Terminal};

use crate::{std_provider::StdProvider, Error, Result};

pub fn copy() -> Result<()> {
    let mut data = vec![];
    stdin().read_to_end(&mut data)?;

    copy_data(&data)
}

pub fn copy_data(data: &[u8]) -> Result<()> {
    // Try do both, on wayland, copy_data_direct may not properly retain the
    // clipboard, but `copy_data_term` is unrelayable because if the terminal
    // doesn't support it, it will fail silently. (also it will not work if
    // both stdin and stderr are not terminal)
    let e = copy_data_term(data);
    copy_data_direct(data).map_err(|e2| {
        if let Err(e) = e {
            Error::Double(Box::new((e, e2)))
        } else {
            e2
        }
    })
}

pub fn copy_data_term(data: &[u8]) -> Result<()> {
    let mut term = Terminal::<StdProvider>::default();

    if !term.is_out_terminal() {
        return Err(Error::NoClipboardProvider);
    }

    term.write_all(codes::set_selection([], data).as_bytes())?;

    Ok(())
}

pub fn copy_data_direct(data: &[u8]) -> Result<()> {
    let mut cb = Clipboard::new()?;
    cb.set_text(std::str::from_utf8(data)?)?;
    Ok(())
}
