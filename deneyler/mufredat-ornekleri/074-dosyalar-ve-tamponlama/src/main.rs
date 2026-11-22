use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let kip = std::env::args().nth(1).expect("kip: tamponsuz | tamponlu");
    let dosya = File::create(format!("/tmp/{kip}.txt")).unwrap();
    if kip == "tamponsuz" {
        let mut f = dosya;
        for i in 0..100_000 {
            writeln!(f, "satır {i}").unwrap(); // her parça doğrudan çekirdeğe
        }
    } else {
        let mut f = BufWriter::new(dosya); // 8 KiB'lik tampon (varsayılan)
        for i in 0..100_000 {
            writeln!(f, "satır {i}").unwrap();
        }
        f.flush().unwrap();
    }
}
