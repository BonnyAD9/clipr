use std::io::{stdin, stdout, Read, Write};

use crate::{copy_data, Result};

pub fn pass() -> Result<()> {
    let mut data = vec![];
    stdin().read_to_end(&mut data)?;
    copy_data(&data)?;
    let mut out = stdout().lock();
    out.write_all(&data)?;
    out.flush()?;
    Ok(())
}
