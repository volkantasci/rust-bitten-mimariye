// Açık kuyruk çağrısı (`become`): çağrı, mevcut yığın çerçevesini yeniden kullanır
fn say(n: u64, birikim: u64) -> u64 {
    if n == 0 {
        return birikim;
    }
    become say(n - 1, birikim + 1)
}

fn main() {
    println!("{}", say(1_000_000, 0));
}
