use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let kip = std::env::args().nth(1).expect("kip: tamponsuz | tamponlu | tamponlu64");
    let dosya = File::create(format!("/tmp/{kip}.txt")).unwrap();
    match kip.as_str() {
        "tamponsuz" => {
            let mut f = dosya; // File: her yazma doğrudan çekirdeğe gider
            for i in 0..100_000 {
                writeln!(f, "satır {i}").unwrap();
            }
        }
        "tamponlu" | "tamponlu64" => {
            let kapasite = if kip == "tamponlu" { 8 * 1024 } else { 64 * 1024 };
            let mut f = BufWriter::with_capacity(kapasite, dosya); // varsayılan BufWriter::new de 8 KiB
            for i in 0..100_000 {
                writeln!(f, "satır {i}").unwrap();
            }
            f.flush().unwrap(); // tampondaki son baytlar; hata olursa burada görülür
        }
        _ => panic!("bilinmeyen kip"),
    }
}
