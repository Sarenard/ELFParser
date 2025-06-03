use std::fs::File;
use std::io::{self, Read, Cursor};


pub mod elf;

use elf::Elf;

fn main() {
    let mut file = File::open("tests/main.bin").unwrap();

    let elf = Elf::new(file);

    println!("Taille du fichier : {} octets", elf.rawbytes.len());

    // for (i, byte) in elf.rawbytes.iter().enumerate() {
    //     println!("Octet {} : {:02X}", i, byte);
    // }

    let mut cursor = Cursor::new(elf.rawbytes);

    let mut byte = [0u8; 1];
    cursor.read_exact(&mut byte).unwrap();
    println!("Premier octet : {:02X}", byte[0]);
}