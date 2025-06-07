use std::io::{Cursor, Seek};
use byteorder::{LittleEndian, ReadBytesExt};

use super::Elf;

pub type ElfHalf = u16;
pub type ElfWord = u32;
pub type ElfAddr = u32; // ADDR
pub type ElfOff = u32; // OFFSET

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Elf32Shdr {
    sh_name: ElfWord,
    sh_type: ElfWord,
    sh_flags: ElfWord,
    sh_addr: ElfAddr,
    sh_offset: ElfOff,
    sh_size: ElfWord,
    sh_link: ElfWord,
    sh_info: ElfWord,
    sh_addralign: ElfWord,
    sh_entsize: ElfWord,
}

impl Elf32Shdr {
    pub fn parse(elf: &Elf, cursor: &mut Cursor<Vec<u8>>) -> std::io::Result<Vec<Self>> {
        
        cursor.seek(std::io::SeekFrom::Start(elf.header.unwrap().e_shoff as u64))?;

        let mut sections: Vec<Self> = vec![];

        for _ in 0..elf.header.unwrap().e_shnum {
            let sh_name = cursor.read_u32::<LittleEndian>()?;
            let sh_type = cursor.read_u32::<LittleEndian>()?;
            let sh_flags = cursor.read_u32::<LittleEndian>()?;
            let sh_addr = cursor.read_u32::<LittleEndian>()?;
            let sh_offset = cursor.read_u32::<LittleEndian>()?;
            let sh_size = cursor.read_u32::<LittleEndian>()?;
            let sh_link = cursor.read_u32::<LittleEndian>()?;
            let sh_info = cursor.read_u32::<LittleEndian>()?;
            let sh_addralign = cursor.read_u32::<LittleEndian>()?;
            let sh_entsize = cursor.read_u32::<LittleEndian>()?;

            let shdr = Elf32Shdr {
                sh_name,
                sh_type,
                sh_flags,
                sh_addr,
                sh_offset,
                sh_size,
                sh_link,
                sh_info,
                sh_addralign,
                sh_entsize,
            };

            sections.push(shdr);
        }

        Ok(sections)
    }
}