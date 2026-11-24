fn main() {
    let n: u32 = 1_000_000;
    println!("onluk         : {n}");
    println!("ikilik        : {n:032b}");
    println!("onaltılık     : {n:#010x}");
    println!("bellekte      : {:02x?}", n.to_ne_bytes()); // bu makinenin kendi sırası
    println!("little-endian : {:02x?}", n.to_le_bytes()); // küçüğü başta
    println!("big-endian    : {:02x?}", n.to_be_bytes()); // büyüğü başta
    let harf = b'A';
    println!("'A' baytı     : {harf} = {harf:#04x} = {harf:08b}");
}
