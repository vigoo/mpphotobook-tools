mod errors;
mod report;

use crate::errors::*;
use crate::report::*;

use error_chain::quick_main;
use std::env;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs::read_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn process_root<P: AsRef<Path> + Debug>(root: P) -> Result<()> {
    println!("Processing {:?}", root);

    let mut out = Vec::new();

    for path_result in read_dir(&root)? {
        let path = path_result?;
        let typ = path.file_type()?;

        if typ.is_dir() {
            let _ = process_root(path.path())?;

        } else {
            if path.path().extension() == Some(OsStr::new("txt")) {
                let report = Report::parse(path.path())?;
                report.write_to_csv(&mut out)?;
            }
        }
    }

    if !out.is_empty() {
        let mut csv = File::create(root.as_ref().join("data.csv"))?;
        let _ = csv.write(&out)?;
    }

    Ok(())
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_, rootdir] => process_root(rootdir),
        _ => {
            eprintln!("Usage: mpphotobook-tools <rootdir>");
            Ok(())
        }
    }
}

quick_main!(run);
