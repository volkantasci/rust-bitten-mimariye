#[inline(never)]
fn asagi(seviye: u32, onceki: usize) {
    let yerel = seviye; // bu çerçevedeki bir yerel değişken
    let adres = &yerel as *const u32 as usize;
    if seviye == 0 {
        println!("seviye {seviye}: yerel @ {adres:#x}");
    } else {
        println!("seviye {seviye}: yerel @ {adres:#x}  ({} bayt aşağıda)", onceki - adres);
    }
    if seviye < 4 {
        asagi(seviye + 1, adres);
    }
}

fn main() {
    asagi(0, 0);
}
