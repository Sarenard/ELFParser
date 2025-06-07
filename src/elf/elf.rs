use std::fs::File;
use std::io::{Cursor, Read};

use crate::elf::progheader::Elf32Phdr;
use crate::elf::secheader::Elf32Shdr;

use super::elfheader::Elf32Header;

#[derive(Debug, Clone)]
pub struct Elf {
    pub rawbytes: Vec<u8>,
    pub header: Option<Elf32Header>,
    pub shdr: Option<Vec<Elf32Shdr>>,
    pub phdr: Option<Vec<Elf32Phdr>>,
}

impl Elf {
    pub fn new(mut file: File) -> Elf {
        let mut buffer: Vec<u8> = Vec::new();

        file.read_to_end(&mut buffer).unwrap();

        Elf::parse(buffer).unwrap()
    }

    fn parse(rawbytes: Vec<u8>) -> std::io::Result<Self> {
        // parse header
        let mut cursor = Cursor::new(rawbytes.clone());
        
        let mut elf = Elf {
            rawbytes,
            header: None,
            shdr: None,
            phdr: None,
        };

        // parse header
        let header = Elf32Header::parse(&elf, &mut cursor)?;
        elf.header = Some(header);
        // println!("{:?}", header);

        // parse section headers
        let sections = Elf32Shdr::parse(&elf, &mut cursor)?;
        elf.shdr = Some(sections);
        // for i in elf.clone().shdr.unwrap() {println!("{:?}", i);}

        // parse program headers
        let progs = Elf32Phdr::parse(&elf, &mut cursor)?;
        elf.phdr = Some(progs);
        // for i in elf.clone().phdr.unwrap() {println!("{:?}", i);}

        // return the elf
        Ok(elf)
    }
}
