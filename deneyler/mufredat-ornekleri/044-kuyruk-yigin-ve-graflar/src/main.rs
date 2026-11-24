use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

const YONLER: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // sağ, aşağı, sol, yukarı

fn komsular(izgara: &[Vec<u8>], (y, x): (usize, usize)) -> Vec<(usize, usize)> {
    let (h, w) = (izgara.len() as i32, izgara[0].len() as i32);
    YONLER
        .iter()
        .map(|&(dy, dx)| (y as i32 + dy, x as i32 + dx))
        .filter(|&(ny, nx)| ny >= 0 && nx >= 0 && ny < h && nx < w)
        .map(|(ny, nx)| (ny as usize, nx as usize))
        .filter(|&(ny, nx)| izgara[ny][nx] != b'#')
        .collect()
}

// Bulunan yolun adım sayısı: hedeften başlangıca önceki[] zincirini izle
fn yol_uzunlugu(onceki: &[Vec<Option<(usize, usize)>>], hedef: (usize, usize)) -> usize {
    let (mut adim, mut p) = (0, hedef);
    while let Some(q) = onceki[p.0][p.1] {
        adim += 1;
        p = q;
    }
    adim
}

fn main() {
    let labirent = ["S.#.....", ".##.###.", "....#...", "#.#...#E"];
    let izgara: Vec<Vec<u8>> = labirent.iter().map(|s| s.bytes().collect()).collect();
    let (h, w) = (izgara.len(), izgara[0].len());
    let (bas, hedef) = ((0, 0), (h - 1, w - 1));

    // 1) Genişlik öncelikli arama (BFS): kuyruk (VecDeque, dairesel tampon) — ilk giren ilk çıkar
    let mut onceki = vec![vec![None; w]; h];
    let mut goruldu = vec![vec![false; w]; h];
    let mut kuyruk = VecDeque::from([bas]);
    goruldu[0][0] = true;
    let mut ziyaret = 0;
    while let Some(p) = kuyruk.pop_front() {
        ziyaret += 1;
        if p == hedef { break; }
        for q in komsular(&izgara, p) {
            if !goruldu[q.0][q.1] {
                goruldu[q.0][q.1] = true;
                onceki[q.0][q.1] = Some(p);
                kuyruk.push_back(q);
            }
        }
    }
    println!("BFS (kuyruk): S → E yolu {} adım, {} hücre ziyaret edildi", yol_uzunlugu(&onceki, hedef), ziyaret);

    // 2) Derinlik öncelikli arama (DFS): yığın (Vec) — son giren ilk çıkar
    let mut onceki = vec![vec![None; w]; h];
    let mut goruldu = vec![vec![false; w]; h];
    let mut yigin = vec![bas];
    goruldu[0][0] = true;
    let mut ziyaret = 0;
    while let Some(p) = yigin.pop() {
        ziyaret += 1;
        if p == hedef { break; }
        for q in komsular(&izgara, p) {
            if !goruldu[q.0][q.1] {
                goruldu[q.0][q.1] = true;
                onceki[q.0][q.1] = Some(p);
                yigin.push(q);
            }
        }
    }
    println!("DFS (yığın) : S → E yolu {} adım, {} hücre ziyaret edildi", yol_uzunlugu(&onceki, hedef), ziyaret);

    // 3) Dijkstra: öncelik kuyruğu (BinaryHeap, ikili öbek; Reverse ile en küçük mesafe önce)
    let sehir = ["İstanbul", "Bursa", "Eskişehir", "Ankara", "Bolu"];
    let yollar = [(0, 1, 155), (0, 4, 260), (1, 2, 150), (2, 3, 235), (4, 3, 190), (1, 4, 210)];
    let mut komsu = vec![Vec::new(); sehir.len()];
    for &(a, b, km) in &yollar {
        komsu[a].push((b, km));
        komsu[b].push((a, km));
    }
    let mut en_kisa = vec![u32::MAX; sehir.len()];
    let mut onceki_sehir = vec![usize::MAX; sehir.len()];
    let mut oncelik = BinaryHeap::from([Reverse((0u32, 0usize))]);
    en_kisa[0] = 0;
    while let Some(Reverse((d, u))) = oncelik.pop() {
        if d > en_kisa[u] { continue; }
        for &(v, km) in &komsu[u] {
            if d + km < en_kisa[v] {
                en_kisa[v] = d + km;
                onceki_sehir[v] = u;
                oncelik.push(Reverse((d + km, v)));
            }
        }
    }
    let mut yol = vec![3];
    while *yol.last().unwrap() != 0 { yol.push(onceki_sehir[*yol.last().unwrap()]); }
    let yol: Vec<_> = yol.iter().rev().map(|&i| sehir[i]).collect();
    println!("Dijkstra (öncelik kuyruğu): İstanbul → Ankara {} km, yol: {}", en_kisa[3], yol.join(" → "));
}
