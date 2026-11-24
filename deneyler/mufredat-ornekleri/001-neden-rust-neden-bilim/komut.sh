#@ video: 001-neden-rust-neden-bilim
#@ degisken: adresler her çalıştırmada değişir (ASLR); miri çıktısındaki alloc numaraları
# 1) Güvenli Rust: iade edilen blok bir sonraki isteğe yeniden verilir (adresler yalnızca sayı olarak karşılaştırılıyor)
cd yeniden-kullanim
k cargo build -q
k ./target/debug/yeniden_kullanim
ks 'for i in $(seq 100); do ./target/debug/yeniden_kullanim | tail -n 1; done | sort | uniq -c'
cd ..
# 2) unsafe Rust: sarkan işaretçiyle okumak tanımsız davranıştır; sonuç derleme ayarına göre değişebilir
cd sarkan
k cargo build -q
k ./target/debug/sarkan
k cargo build -q --release
k ./target/release/sarkan
# 3) miri programı yorumlayarak çalıştırır ve tanımsız davranışı yakalar
k cargo +nightly miri run -q
cd ..
# 4) Güvenli Rust: aynı desen derlenmez (E0505)
cd odunc && k cargo build -q; cd ..
# 5) Kendin dene: ödüncün son kullanımı iadeden önceye alınınca program derlenir
cd dogru-sira && k cargo run -q; cd ..
# 6) Derleyicinin önerisi (help: clone) uygulanınca program derlenir; referans artık bir kopyaya bakar
cd klon && k cargo run -q; cd ..
