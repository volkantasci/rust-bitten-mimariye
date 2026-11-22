use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    let kip = std::env::args().nth(1).unwrap_or_default();
    if kip == "cok-yaz" {
        for i in 0..100_000 {
            println!("satır {i}");
        }
        return;
    }
    // 1) Boru: çocuk sürecin stdin'ine yaz, stdout'unu oku
    let mut sirala = Command::new("sort").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();
    sirala.stdin.take().unwrap().write_all("zeytin\nelma\nkiraz\n".as_bytes()).unwrap(); // stdin kapanır: EOF
    let cikti = sirala.wait_with_output().unwrap();
    print!("sort çıktısı:\n{}", String::from_utf8_lossy(&cikti.stdout));

    // 2) Sinyaller: çocuğa SIGTERM ve SIGKILL
    let mut uyuyan = Command::new("sleep").arg("10").spawn().unwrap();
    thread::sleep(Duration::from_millis(100));
    Command::new("kill").arg("-TERM").arg(uyuyan.id().to_string()).status().unwrap();
    println!("SIGTERM sonrası: {}", uyuyan.wait().unwrap());
    let mut ikinci = Command::new("sleep").arg("10").spawn().unwrap();
    ikinci.kill().unwrap(); // Child::kill → SIGKILL
    println!("kill() sonrası  : {}", ikinci.wait().unwrap());
}
