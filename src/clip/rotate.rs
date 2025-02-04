use crate::Result;

use super::{copy, paste};

pub fn rotate() -> Result<()> {
    paste()?;
    copy()
}
