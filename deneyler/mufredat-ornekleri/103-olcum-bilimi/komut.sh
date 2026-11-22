#@ video: 103-olcum-bilimi
#@ degisken: süreler
# Tuzak 1: ilk sürümümüzde satır/sütun gezinme süreleri neredeyse aynı çıktı (045'in ilk hâli)
cd tuzak && k cargo run -q --release; cd ..
# Aynı kod, tek fark: matris boyutu n derleme anında bilinmiyor (black_box)
cd tuzak-n-gizli && ks 'cargo run -q --release | grep matris'; cd ..
# Tuzak 2: sonucu kullanılmayan hesap ölçülemez
cd black-box && k cargo run -q --release
# İstatistik: tek ölçüm değil, dağılım (hyperfine: ısınma, ortalama ± σ, aralık)
k cargo build -q --release
k hyperfine -N --warmup 3 --runs 30 --style basic ./target/release/black_box
