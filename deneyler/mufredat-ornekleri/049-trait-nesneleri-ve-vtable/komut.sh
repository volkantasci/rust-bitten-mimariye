#@ video: 049-trait-nesneleri-ve-vtable
#@ degisken: adresler (ASLR; Miri'nin sanal adresleri de sürüme bağlı)
cd vtable && k cargo run -q; cd ..
# Aynı program Miri'de (Rust'ın soyut makinesini yorumlayan araç): vtable'ı sıradan bellek gibi okumak tanımsız davranış
cd vtable && k cargo +nightly miri run -q; cd ..
cd dyn-uyumsuz && k cargo build -q; cd ..
