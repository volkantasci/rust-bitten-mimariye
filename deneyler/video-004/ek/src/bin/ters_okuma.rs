// Aynı dört bayt iki farklı sırayla okununca
fn main() {
    let baytlar: [u8; 4] = 1_000_000u32.to_ne_bytes(); // bellekteki sıra (makinenin kendi bayt sırası)
    println!("baytlar        : {baytlar:02x?}");
    println!("from_le_bytes  : {}", u32::from_le_bytes(baytlar)); // küçüğü başta okur
    println!("from_be_bytes  : {}", u32::from_be_bytes(baytlar)); // büyüğü başta okur
    println!("  onaltılık    : {:#010x}", u32::from_be_bytes(baytlar));
}
