use anyhow::{bail, Result};
use std::{io::ErrorKind, str};

pub fn take_while<F>(from: &str, mut pred: F) -> Result<(&str, usize)>
where
    F: FnMut(char) -> bool,
{
    let mut curr_index = 0;

    for ch in from.chars() {
        let will_continue = pred(ch);

        if !will_continue {
            break;
        }

        curr_index += ch.len_utf8();
    }

    if curr_index == 0 {
        bail!(ErrorKind::InvalidInput)
    } else {
        Ok((&from[..curr_index], curr_index))
    }
}
