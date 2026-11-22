fn topla(v: &[i32]) -> i32 {
    let mut s = 0;
    for x in v {
        s += x;
    }
    s
}

fn oku(metin: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = metin.parse()?;
    Ok(n + 1)
}

fn main() {
    println!("{} {:?}", topla(&[1, 2, 3]), oku("41"));
}
