mod crossterm;
mod app;
mod ui;

use crate::crossterm::run;
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    run()?;
    Ok(())
}