#@ video: 093-ffi-c-ile-konusmak
#@ degisken: sembol adresleri; v0 adındaki crate özeti (Cs…_) derleme ayarlarına göre değişebilir; yanlış beyanla okunan topla değeri tanımsızdır
k cargo run -q
# İkilide iki tarafın sembolleri yan yana: no_mangle ile adlar olduğu gibi kalır
ks 'nm target/debug/ffi | grep -E " T (topla|selamla|karesi_arti_bir|rust_kare)$"'
# no_mangle olmayan bir Rust fonksiyonu (main): ad karıştırma, v0 biçimi (_R ile başlar)
ks 'nm target/debug/ffi | grep -E "_R[A-Za-z0-9_]*3ffi4main$"'
# Çağırma kuralı makine kodunda (x86-64 System V): ilk tamsayı argüman edi'de, dönüş değeri eax'te
k cargo build -q --release
ks 'objdump -d -M intel --no-show-raw-insn target/release/ffi | awk "/<rust_kare>:/,/ret/"'
ks 'objdump -d -M intel --no-show-raw-insn target/release/ffi | awk "/<karesi_arti_bir>:/,/ret/"'
# Kendin dene: no_mangle satırı silinince C tarafı rust_kare adını bulamaz (bağlama hatası)
ks 'rm -rf /tmp/ffi-kopya && mkdir /tmp/ffi-kopya && cp -r Cargo.toml Cargo.lock build.rs c src /tmp/ffi-kopya/ && cd /tmp/ffi-kopya && sed -i "/no_mangle/d" src/main.rs && cargo build -q 2>&1 | grep -E "undefined symbol|^error: linking"'
# Kendin dene: Rust tarafındaki topla beyanını f64 döndürür yapmak. rustc C kaynağını görmediği için itiraz etmez;
# kayan noktalı dönüş değeri xmm0'dan okunur, C ise sonucu eax'e koymuştu: anlamsız sayı (tanımsız davranış, sizde farklı)
ks 'rm -rf /tmp/ffi-kopya2 && mkdir /tmp/ffi-kopya2 && cp -r Cargo.toml Cargo.lock build.rs c src /tmp/ffi-kopya2/ && cd /tmp/ffi-kopya2 && sed -i "s/fn topla(a: c_int, b: c_int) -> c_int;/fn topla(a: c_int, b: c_int) -> f64;/" src/main.rs && cargo run -q 2>&1 | head -n 1'
