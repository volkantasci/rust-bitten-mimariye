#@ video: 067-veri-yarisi-ve-send-sync
#@ degisken: yarışlı sayacın sonucu ve adresler her çalıştırmada değişir
# 1) unsafe Rust ile bilerek yazılmış veri yarışı: kaybolan artışlar
cd yaris
k cargo build -q --release
k ./target/release/yaris
k ./target/release/yaris
# Makine kodu: belleği doğrudan artıran tek bir inc komutu (lock öneki yok)
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/yaris | grep -A12 "<yaris::artir>:" | grep -m1 "inc"'
# miri (Rust'ın tanımsız davranış denetleyicisi) aynı programı yorumlayarak çalıştırır
ks 'cargo +nightly miri run -q -- 100 2>&1 | head -n 12'
cd ..
# 2) Güvenli Rust: aynı paylaşım denemeleri derlenmez
cd ortak-mut && k cargo build -q; cd ..
cd rc-gonderilemez && k cargo build -q; cd ..
# 3) Atomik sayaç: doğru sonuç; makine kodunda aynı inc komutu, bu kez lock önekiyle
cd atomik
k cargo run -q --release
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/atomik | grep -A12 "<atomik::artir>:" | grep -m1 lock'
cd ..
# Kendin dene: rc-gonderilemez/'de Rc → Arc, RefCell → Mutex, borrow_mut()/borrow() → lock().unwrap()
cd arc-mutex && k cargo run -q; cd ..
