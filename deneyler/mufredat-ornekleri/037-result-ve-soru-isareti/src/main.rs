use std::num::ParseIntError;

fn ikiye_katla(metin: &str) -> Result<i32, ParseIntError> {
    let n: i32 = metin.trim().parse()?; // hata varsa hemen geri döndür
    Ok(n * 2)
}

fn main() -> Result<(), ParseIntError> {
    println!("{:?}", ikiye_katla("21"));
    println!("{:?}", ikiye_katla("yirmi bir"));
    println!("{:?}", "99999999999".parse::<i32>());
    let toplam: i32 = ["1", "2", "3"].iter().map(|s| s.parse::<i32>()).sum::<Result<i32, _>>()?;
    println!("toplam = {toplam}");
    let x = ikiye_katla("x")?; // main de Result döndürür: hata çıkış koduna dönüşür
    println!("buraya gelinmez: {x}");
    Ok(())
}
