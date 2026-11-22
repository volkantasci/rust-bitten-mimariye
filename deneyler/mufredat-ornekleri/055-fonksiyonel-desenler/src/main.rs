use std::collections::BTreeMap;

fn main() {
    let tekerleme = "bir berber bir berbere gel beraber bir berber dükkanı açalım demiş";
    let sayim = tekerleme.split_whitespace().fold(BTreeMap::new(), |mut harita, kelime| {
        *harita.entry(kelime).or_insert(0) += 1;
        harita
    });
    println!("{sayim:?}");

    let girdiler = ["10", "20", "otuz", "40"];
    let hepsi: Result<Vec<i32>, _> = girdiler.iter().map(|s| s.parse::<i32>()).collect();
    println!("hepsi ya da hiç : {hepsi:?}");
    let gecerliler: Vec<i32> = girdiler.iter().filter_map(|s| s.parse().ok()).collect();
    println!("geçerli olanlar : {gecerliler:?}");
    let (cift, tek): (Vec<i32>, Vec<i32>) = (1..=10).partition(|x| x % 2 == 0);
    println!("çift {cift:?}, tek {tek:?}");
}
