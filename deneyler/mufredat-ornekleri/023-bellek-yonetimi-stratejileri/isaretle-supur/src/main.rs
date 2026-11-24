// İzleyen çöp toplama, küçük bir benzetim: işaretle ve süpür.
// Nesneler bir vektörde durur; bağlar indislerdir. Kökler programın doğrudan eriştikleridir.
struct Nesne {
    ad: char,
    baglar: Vec<usize>,
}

fn topla(obek: &mut Vec<Option<Nesne>>, kokler: &[usize]) {
    // 1) İşaretle: köklerden başlayıp bağları izle
    let mut isaretli = vec![false; obek.len()];
    let mut bekleyen: Vec<usize> = kokler.to_vec();
    while let Some(i) = bekleyen.pop() {
        if isaretli[i] {
            continue;
        }
        isaretli[i] = true;
        if let Some(n) = &obek[i] {
            bekleyen.extend(&n.baglar);
        }
    }
    // 2) Süpür: işaretlenmemiş her nesneyi sil
    for (i, yer) in obek.iter_mut().enumerate() {
        if !isaretli[i] {
            if let Some(n) = yer.take() {
                println!("  süpürüldü: {}", n.ad);
            }
        }
    }
}

fn main() {
    let mut obek = vec![
        Some(Nesne { ad: 'A', baglar: vec![1] }), // 0: kök
        Some(Nesne { ad: 'B', baglar: vec![2] }), // 1
        Some(Nesne { ad: 'C', baglar: vec![] }),  // 2
        Some(Nesne { ad: 'D', baglar: vec![4] }), // 3: D -> E
        Some(Nesne { ad: 'E', baglar: vec![3] }), // 4: E -> D (çevrim)
        Some(Nesne { ad: 'F', baglar: vec![] }),  // 5
    ];
    let kokler = [0];
    println!("toplama başlıyor (kök: A)");
    topla(&mut obek, &kokler);
    let kalan: Vec<char> = obek.iter().flatten().map(|n| n.ad).collect();
    println!("kalanlar: {kalan:?}");
}
