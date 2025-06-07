use std::fs::File;
use std::io::{Cursor, Read};

use crate::elf::sections::Elf32Shdr;

use super::elfheader::Elf32Header;

#[derive(Debug, Clone)]
pub struct Elf {
    pub rawbytes: Vec<u8>,
    pub header: Option<Elf32Header>,
    pub sechdr: Option<Vec<Elf32Shdr>>,
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
            sechdr: None,
        };

        let header = Elf32Header::parse(&elf, &mut cursor)?;
        println!("header :\n{:?}", header);

        elf.header = Some(header);

        // parse sections
        let nb_sections = header.e_shnum;
        println!("Il y a {} sections, traitons les", nb_sections);
        let sections = Elf32Shdr::parse(&elf, &mut cursor)?;
        elf.sechdr = Some(sections);

        for i in elf.clone().sechdr.unwrap() {
            println!("{:?}", i);
        }

        // parse segments
        // let nb_segments = header.e_phnum;
        // println!("Il y a {} segments", nb_segments);

        // return the elf

        Ok(elf)
    }
}
