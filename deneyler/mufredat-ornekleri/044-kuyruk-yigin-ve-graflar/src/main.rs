use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

fn main() {
    // 1) Genişlik öncelikli arama (BFS) — kuyruk: VecDeque
    let labirent = ["S.#.....", ".##.###.", "....#...", "#.#...#E"];
    let izgara: Vec<Vec<u8>> = labirent.iter().map(|s| s.bytes().collect()).collect();
    let (h, w) = (izgara.len(), izgara[0].len());
    let mut mesafe = vec![vec![usize::MAX; w]; h];
    let mut kuyruk = VecDeque::from([(0usize, 0usize)]);
    mesafe[0][0] = 0;
    while let Some((y, x)) = kuyruk.pop_front() {
        for (dy, dx) in [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)] {
            let (ny, nx) = (y as i32 + dy, x as i32 + dx);
            if ny < 0 || nx < 0 || ny >= h as i32 || nx >= w as i32 { continue; }
            let (ny, nx) = (ny as usize, nx as usize);
            if izgara[ny][nx] != b'#' && mesafe[ny][nx] == usize::MAX {
                mesafe[ny][nx] = mesafe[y][x] + 1;
                kuyruk.push_back((ny, nx));
            }
        }
    }
    println!("BFS: S → E en kısa yol {} adım", mesafe[h - 1][w - 1]);

    // 2) Dijkstra — öncelik kuyruğu: BinaryHeap (Reverse ile en küçük önce)
    let sehir = ["İstanbul", "Bursa", "Eskişehir", "Ankara", "Bolu"];
    let yollar = [(0, 1, 155), (0, 4, 260), (1, 2, 150), (2, 3, 235), (4, 3, 190), (1, 4, 210)];
    let mut komsu = vec![Vec::new(); sehir.len()];
    for &(a, b, km) in &yollar {
        komsu[a].push((b, km));
        komsu[b].push((a, km));
    }
    let mut en_kisa = vec![u32::MAX; sehir.len()];
    let mut onceki = vec![usize::MAX; sehir.len()];
    let mut yigin = BinaryHeap::from([Reverse((0u32, 0usize))]);
    en_kisa[0] = 0;
    while let Some(Reverse((d, u))) = yigin.pop() {
        if d > en_kisa[u] { continue; }
        for &(v, km) in &komsu[u] {
            if d + km < en_kisa[v] {
                en_kisa[v] = d + km;
                onceki[v] = u;
                yigin.push(Reverse((d + km, v)));
            }
        }
    }
    let mut yol = vec![3];
    while *yol.last().unwrap() != 0 { yol.push(onceki[*yol.last().unwrap()]); }
    let yol: Vec<_> = yol.iter().rev().map(|&i| sehir[i]).collect();
    println!("Dijkstra: İstanbul → Ankara {} km, yol: {}", en_kisa[3], yol.join(" → "));
}
