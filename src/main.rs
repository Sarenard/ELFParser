use std::fs::File;
use std::io::{self, Cursor, Read};

pub mod elf;

use elf::Elf;

fn main() {
    let file = File::open("tests/main.out").unwrap(); // 32 bit file

    let elf = Elf::new(file);

    println!("Taille du fichier : {} octets", elf.rawbytes.len());
}
