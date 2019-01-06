use crate::errors::*;

use encoding::all::ISO_8859_2;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use encoding::DecoderTrap;
use encoding::types::Encoding;

fn get_id(path: &Path) -> Result<String> {
    match path.file_stem() {
        Some(name) => Ok(name.to_string_lossy().to_string()),
        None => Err(ErrorKind::InvalidPath.into())
    }
}

#[derive(Debug)]
pub struct Report {
    id: String,
    pages: u32,
    total_price: f32
}

impl Report {
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<Report> {
        let path_ref = path.as_ref();
        println!("Parsing {:?}", path_ref);

        let mut file = File::open(path_ref)?;
        let mut raw: Vec<u8> = Vec::new();
        let _len = file.read_to_end(&mut raw)?;
        let contents = ISO_8859_2.decode(&raw, DecoderTrap::Strict).map_err(|_| ErrorKind::DecodeError)?;

        let mut pages: Option<u32> = None;
        let mut total_price: Option<f32> = None;

        for line in contents.lines() {
            if line.starts_with("Lapok száma:") {
                match line.split_whitespace().last() {
                    Some(s) => pages = Some(s.parse()?),
                    None => {}
                }
            }
            else if line.starts_with("Ár (kézb.nélkül):") {
                match line.split_whitespace().last() {
                    Some(s) => total_price = Some(s.replace(",", ".").parse()?),
                    None => {}
                }
            }
        }

        match (pages, total_price) {
            (Some(p), Some(tp)) =>
                Ok(Report {
                    id: get_id(path_ref)?,
                    pages: p,
                    total_price: tp
                }),
            _ =>
                Err(ErrorKind::MissingData.into())
        }
    }

    pub fn write_to_csv<W: Write>(&self, write: &mut W) -> Result<()> {
        Ok(writeln!(write, "{},{},{}", self.id, self.pages, self.total_price)?)
    }
}