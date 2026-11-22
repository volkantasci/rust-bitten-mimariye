fn ortalama(v: &Vec<i32>) -> f64 {
    let mut toplam = 0;
    for i in 0..v.len() {
        toplam = toplam + v[i];
    }
    if v.len() == 0 {
        return 0.0;
    }
    return toplam as f64 / v.len() as f64;
}

fn main() {
    let   notlar = vec![70, 85,90];
    println!("{}", ortalama(&notlar));
}
