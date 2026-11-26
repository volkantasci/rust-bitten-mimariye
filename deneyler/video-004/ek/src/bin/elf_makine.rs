// Bir ELF dosyasının başlığından üç alan (konumlar /usr/include/elf.h'deki Elf64_Ehdr'den):
// e_ident[EI_DATA] = 6. bayt (bayt sırası), e_type = konum 16-17, e_machine = konum 18-19
use std::{env, fs};

fn main() {
    let yol = env::args().nth(1).expect("kullanım: elf_makine <dosya>");
    let dosya = fs::read(&yol).expect("dosya okunamadı");
    println!("EI_DATA (6. bayt)     : {:02x}", dosya[5]);
    let tur = [dosya[16], dosya[17]];
    println!("e_type baytları       : {tur:02x?}");
    println!("  u16::from_le_bytes  : {}", u16::from_le_bytes(tur));
    let makine = [dosya[18], dosya[19]];
    println!("e_machine baytları    : {makine:02x?}");
    println!("  u16::from_le_bytes  : {}", u16::from_le_bytes(makine));
    println!("  u16::from_be_bytes  : {}", u16::from_be_bytes(makine));
}
