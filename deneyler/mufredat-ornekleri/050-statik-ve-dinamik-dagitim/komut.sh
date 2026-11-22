#@ video: 050-statik-ve-dinamik-dagitim
#@ degisken: süreler
k cargo build -q --release
k ./target/release/dagitim
# Üç fonksiyon yazdık, ikilide iki sembol var: &dyn ve Box<dyn> sürümleri aynı koda derlendi ve birleştirildi
ks 'nm -C target/release/dagitim | grep toplam'
# Dinamik sürümde dolaylı çağrı: adres bellekten (vtable'ın 4. girdisi, +0x18) okunup çağrılıyor
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/dagitim | awk "/<dagitim::toplam_kutulu>:/,/ret/" | grep -E "call|ret"'
# Statik sürümde hiç çağrı yok: alan() satır içine açılmış
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/dagitim | awk "/<dagitim::toplam_statik::<dagitim::Kare>>:/,/ret/" | grep -E "call|ret"'
