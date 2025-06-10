use std::fs::File;

pub mod elf;

use elf::Elf;

fn main() {
    let file = File::open("tests/asm.out").unwrap(); // 32 bit file

    let elf = Elf::new(file);

    println!("{:?}", elf.header.unwrap());

    for x in elf.phdr.unwrap().iter() {
        println!("{:?}", x);
    }

    for x in elf.shdr.unwrap().iter() {
        println!("{:?}", x);
    }

    println!("File size : {} bytes", elf.rawbytes.len());
}
