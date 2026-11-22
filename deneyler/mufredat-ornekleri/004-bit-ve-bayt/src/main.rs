fn main() {
    let n: u32 = 1_000_000;
    println!("onluk      : {n}");
    println!("ikilik     : {n:032b}");
    println!("onaltılık  : {n:#010x}");
    println!("bellekte   : {:02x?}", n.to_ne_bytes()); // bu makinenin kendi sırası
    println!("küçük uçlu : {:02x?}", n.to_le_bytes());
    println!("büyük uçlu : {:02x?}", n.to_be_bytes());
    let harf = b'A';
    println!("'A' baytı  : {harf} = {harf:#04x} = {harf:08b}");
}
