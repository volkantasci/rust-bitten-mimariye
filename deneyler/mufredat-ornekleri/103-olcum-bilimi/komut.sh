#@ video: 103-olcum-bilimi
#@ degisken: süreler
# Tuzak 1: ilk sürümümüzde satır/sütun gezinme süreleri neredeyse aynı çıktı (045'in ilk hâli)
cd tuzak && k cargo run -q --release; cd ..
# Aynı kod, tek fark: matris boyutu n derleme anında bilinmiyor (black_box)
cd tuzak-n-gizli && ks 'cargo run -q --release | grep matris'; cd ..
# Neden? LLVM'in kendi raporu: n bilinince döngü yer değiştirme (LoopInterchange) geçişi kodu değiştirdi, gizlenince değiştirmedi
cd tuzak && ks 'RUSTFLAGS="-C llvm-args=-print-changed=quiet" cargo build -q --release --target-dir target/gecisler 2>&1 | grep -c "IR Dump After LoopInterchangePass" | xargs echo "n bilinirken LoopInterchangePass değişikliği:"'; cd ..
cd tuzak-n-gizli && ks 'RUSTFLAGS="-C llvm-args=-print-changed=quiet" cargo build -q --release --target-dir target/gecisler 2>&1 | grep -c "IR Dump After LoopInterchangePass" | xargs echo "n gizlenince LoopInterchangePass değişikliği:"'; cd ..
# Kendin dene: n bilinirken geçişi kapatmak (LLVM seçeneği); sütun gezinme yine yavaşlar
cd tuzak && ks 'RUSTFLAGS="-C llvm-args=-enable-loopinterchange=false" cargo run -q --release --target-dir target/donusumsuz | grep matris'; cd ..
# Tuzak 2: sonucu kullanılmayan hesap ölçülemez
cd black-box && k cargo run -q --release
# İstatistik: tek ölçüm değil, dağılım (hyperfine: ısınma, ortalama ± σ, aralık)
k cargo build -q --release
k hyperfine -N --warmup 3 --runs 30 --style basic --export-json target/olcum.json ./target/release/black_box
# Otuz ölçümün hepsi (ms, küçükten büyüğe): dağılımı kendimiz görelim
ks 'python3 -c "import json; t=sorted(x*1000 for x in json.load(open(\"target/olcum.json\"))[\"results\"][0][\"times\"]); print(\" \".join(f\"{x:.1f}\" for x in t))"'
