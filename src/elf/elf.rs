use std::fs::File;
use std::io::{Read, Cursor};

use super::elfheader::Elf32Header;

#[derive(Debug)]
pub struct Elf {
    pub rawbytes: Vec<u8>,
    pub header: Elf32Header,
}

impl Elf {
    pub fn new(mut file: File) -> Elf {
        let mut buffer: Vec<u8> = Vec::new();

        file.read_to_end(&mut buffer).unwrap();

        Elf::parse(buffer).unwrap()
    }

    fn parse(rawbytes: Vec<u8>) -> std::io::Result<(Self)> {

        // parse header
        let mut cursor = Cursor::new(rawbytes);

        let header = Elf32Header::parse(&mut cursor)?;

        println!("header :\n{:?}", header);

        let elf = Elf {
            rawbytes: cursor.into_inner(),
            header: header,
        };
        
        Ok(elf)
    }
}