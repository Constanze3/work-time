use std::io::Result;
use std::path::Path;

use crate::client::WorkData;

pub fn work_started<S, P>(client_id: S, output_folder: P) -> Result<()>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    // TODO write to a file that the work has started
    Ok(())
}

pub fn work_ended<S, P>(client_id: S, output_folder: P, data: WorkData) -> Result<()>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    // TODO write to a file that the work has started
    Ok(())
}
