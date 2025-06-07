use std::io::{Cursor, Seek};
use byteorder::{LittleEndian, ReadBytesExt};

use super::Elf;

pub type ElfHalf = u16;
pub type ElfWord = u32;
pub type ElfAddr = u32; // ADDR
pub type ElfOff = u32; // OFFSET

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Elf32Phdr {
    p_type: ElfWord,
    p_offset: ElfOff,
    p_vaddr: ElfAddr,
    p_paddr: ElfAddr,
    p_filesz: ElfWord,
    p_memsz: ElfWord,
    p_flags: ElfWord,
    p_aligh: ElfWord,
}

impl Elf32Phdr {
    pub fn parse(elf: &Elf, cursor: &mut Cursor<Vec<u8>>) -> std::io::Result<Vec<Self>> {
        
        cursor.seek(std::io::SeekFrom::Start(elf.header.unwrap().e_phoff as u64))?;

        let mut progs: Vec<Self> = vec![];

        for _ in 0..elf.header.unwrap().e_phnum {
            let p_type = cursor.read_u32::<LittleEndian>()?;
            let p_offset = cursor.read_u32::<LittleEndian>()?;
            let p_vaddr = cursor.read_u32::<LittleEndian>()?;
            let p_paddr = cursor.read_u32::<LittleEndian>()?;
            let p_filesz = cursor.read_u32::<LittleEndian>()?;
            let p_memsz = cursor.read_u32::<LittleEndian>()?;
            let p_flags = cursor.read_u32::<LittleEndian>()?;
            let p_aligh = cursor.read_u32::<LittleEndian>()?;

            let phdr = Elf32Phdr {
                p_type,
                p_offset,
                p_vaddr,
                p_paddr,
                p_filesz,
                p_memsz,
                p_flags,
                p_aligh,
            };

            progs.push(phdr);
        }

        Ok(progs)
    }
}