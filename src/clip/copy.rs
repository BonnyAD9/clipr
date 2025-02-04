use std::{
    env,
    io::{stdin, Read, Write},
};

use arboard::Clipboard;
use termal::{codes, raw::Terminal};

use crate::{std_provider::StdProvider, Error, Result};

pub fn copy() -> Result<()> {
    let mut data = vec![];
    stdin().read_to_end(&mut data)?;

    copy_data(&data)
}

pub fn copy_data(data: &[u8]) -> Result<()> {
    // On wayland, `copy_data_direct` will propably silently fail, so prefer to
    // use the terminal variant. Otherwise the direct variant is more reliable.
    let session = env::var("XDG_SESSION_TYPE").unwrap_or_default();
    if session == "wayland" {
        copy_data_term(data).or_else(|e| {
            copy_data_direct(data)
                .map_err(|e2| Error::Double(Box::new((e, e2))))
        })
    } else {
        copy_data_direct(data).or_else(|e| {
            copy_data_term(data).map_err(|e2| Error::Double(Box::new((e, e2))))
        })
    }
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
