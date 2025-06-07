use std::fs::File;

pub mod elf;

use elf::Elf;

fn main() {
    let file = File::open("tests/main.out").unwrap(); // 32 bit file

    let elf = Elf::new(file);

    println!("Taille du fichier : {} octets", elf.rawbytes.len());
}
