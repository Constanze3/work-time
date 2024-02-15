use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;
use std::time::SystemTime;

use crate::client::WorkData;

pub fn work_started<S, P>(client_id: S, output_file: P) -> Result<()>
where
    S: AsRef<str> + std::fmt::Display,
    P: AsRef<Path>,
{
    let client_id: &str = client_id.as_ref();
    let output_file: &Path = output_file.as_ref();

    // TODO write to a file that the work has started
    let mut f = File::create(output_file)?;
    write!(f, "{} {:?}", client_id, SystemTime::now())?;

    Ok(())
}

pub fn work_ended<S, P>(client_id: S, output_file: P, data: WorkData) -> Result<()>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    // TODO write to a file that the work has started
    Ok(())
}
