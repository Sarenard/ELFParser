use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};

pub type ElfHalf = u16;
pub type ElfWord = u32;
pub type ElfAddr = u32; // ADDR
pub type ElfOff = u32; // OFFSET

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EType {
    NONE = 0,
    REL = 1,
    EXEC = 2,
    DYN = 3,
    CORE = 4,
    LOPROC = 0xff00,
    HIPROC = 0xffff,
}

impl From<u16> for EType {
    fn from(value: u16) -> Self {
        match value {
            0 => EType::NONE,
            1 => EType::REL,
            2 => EType::EXEC,
            3 => EType::DYN,
            4 => EType::CORE,
            0xff00 => EType::LOPROC,
            0xffff => EType::HIPROC,
            _ => panic!("Valeur inconnue pour EType: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EMachine {
    NONE,      // No machine
    M32,       // AT&T WE 32100
    SPARC,     // SPARC
    INTEL386,  // Intel Architecture
    M68K,      // Motorola 68000
    M88K,      // Motorola 88000
    INTEL860,  // Intel 80860
    MIPS,      // MIPS RS3000 Big-Endian
    MIPSRS4BE, // MIPS RS4000 Big-Endian
    // Reserved 11-16
    Other(u16),
}

impl From<u16> for EMachine {
    fn from(value: u16) -> Self {
        match value {
            0 => EMachine::NONE,
            1 => EMachine::M32,
            2 => EMachine::SPARC,
            3 => EMachine::INTEL386,
            4 => EMachine::M68K,
            5 => EMachine::M88K,
            7 => EMachine::INTEL860,
            8 => EMachine::MIPS,
            10 => EMachine::MIPSRS4BE,
            // 11-16 réservés, on considère comme inconnus
            other => EMachine::Other(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVersion {
    NONE,         // invalid version
    CURRENT(u32), // current version
}

impl From<u32> for EVersion {
    fn from(value: u32) -> Self {
        match value {
            0 => EVersion::NONE,
            v => EVersion::CURRENT(v),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Elf32HeaderIdent {
    pub mag0: u8,
    pub mag1: u8,
    pub mag2: u8,
    pub mag3: u8,
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub pad0: u8,
    pub pad1: u8,
    pub pad2: u8,
    pub pad3: u8,
    pub pad4: u8,
    pub pad5: u8,
    pub pad6: u8,
    pub pad7: u8,
    pub pad8: u8,
}

impl Elf32HeaderIdent {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> std::io::Result<(Self)> {
        let mut buf = [0u8; 16];

        cursor.read_exact(&mut buf)?;

        assert!(buf[0] == 0x7f);
        assert!(buf[1] == 'E' as u8);
        assert!(buf[2] == 'L' as u8);
        assert!(buf[3] == 'F' as u8);
        assert!([0, 1, 2].contains(&buf[4]));
        assert!([0, 1, 2].contains(&buf[5]));

        Ok(Elf32HeaderIdent {
            mag0: buf[0],
            mag1: buf[1],
            mag2: buf[2],
            mag3: buf[3],
            class: buf[4],
            data: buf[5],
            version: buf[6],
            pad0: buf[7],
            pad1: buf[8],
            pad2: buf[9],
            pad3: buf[10],
            pad4: buf[11],
            pad5: buf[12],
            pad6: buf[13],
            pad7: buf[14],
            pad8: buf[15],
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Elf32Header {
    pub e_ident: Elf32HeaderIdent,
    pub e_type: EType,
    pub e_machine: EMachine,
    pub e_version: EVersion,
    pub e_entry: ElfAddr,
    pub e_phoff: ElfOff,
    pub e_shoff: ElfOff,
    pub e_flags: ElfWord,
    pub e_ehsize: ElfHalf,
    pub e_phentsize: ElfHalf,
    pub e_phnum: ElfHalf,
    pub e_shentsize: ElfHalf,
    pub e_shnum: ElfHalf,
    pub e_shstrndx: ElfHalf, // means no string section
}

impl Elf32Header {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> std::io::Result<(Self)> {
        // On commence par parser l'ident
        let e_ident = Elf32HeaderIdent::parse(cursor)?;
        println!("{:?}", e_ident);
        // Puis on lit les autres champs (en little endian ici, adapter si besoin)
        let e_type_raw = cursor.read_u16::<LittleEndian>()?;
        println!("e_type_raw {}", e_type_raw);
        let e_machine_raw = cursor.read_u16::<LittleEndian>()?;
        println!("e_machine_raw {}", e_machine_raw);
        let e_version_raw = cursor.read_u32::<LittleEndian>()?;
        let e_entry = cursor.read_u32::<LittleEndian>()?;
        let e_phoff = cursor.read_u32::<LittleEndian>()?;
        let e_shoff = cursor.read_u32::<LittleEndian>()?;
        let e_flags = cursor.read_u32::<LittleEndian>()?;
        let e_ehsize = cursor.read_u16::<LittleEndian>()?;
        let e_phentsize = cursor.read_u16::<LittleEndian>()?;
        let e_phnum = cursor.read_u16::<LittleEndian>()?;
        let e_shentsize = cursor.read_u16::<LittleEndian>()?;
        let e_shnum = cursor.read_u16::<LittleEndian>()?;
        let e_shstrndx = cursor.read_u16::<LittleEndian>()?;

        // TODO: Convertir les valeurs brutes en types spécifiques (EType, EMachine, EVersion)
        // Par exemple : let e_type = EType::from(e_type_raw);

        Ok(Self {
            e_ident,
            e_type: EType::from(e_type_raw),
            e_machine: EMachine::from(e_machine_raw),
            e_version: EVersion::from(e_version_raw),
            e_entry,
            e_phoff,
            e_shoff,
            e_flags,
            e_ehsize,
            e_phentsize,
            e_phnum,
            e_shentsize,
            e_shnum,
            e_shstrndx,
        })
    }
}
