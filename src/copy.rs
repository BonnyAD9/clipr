use std::io::{stdin, Read, Write};

use termal::{codes, raw::Terminal};

use crate::{std_provider::StdProvider, Error, Result};

pub fn copy() -> Result<()> {
    let mut data = vec![];
    stdin().read_to_end(&mut data)?;

    copy_data(&data)
}

pub fn copy_data(data: &[u8]) -> Result<()> {
    let mut term = Terminal::<StdProvider>::default();

    if !term.is_out_terminal() {
        return Err(Error::NoClipboardProvider);
    }

    term.write_all(codes::set_selection([], data).as_bytes())?;

    Ok(())
}
