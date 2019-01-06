mod errors;
mod report;

use crate::errors::*;
use crate::report::*;

use error_chain::quick_main;
use std::fs::read_dir;

fn run() -> Result<()> {
    let mut out = std::io::stdout();

    for path_result in read_dir("testdata")? {
        let path = path_result?;
        let report = Report::parse(path.path())?;

        report.write_to_csv(&mut out)?;
    }

    Ok(())
}

quick_main!(run);
