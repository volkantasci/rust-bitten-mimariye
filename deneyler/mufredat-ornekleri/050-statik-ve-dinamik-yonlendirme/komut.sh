#@ video: 050-statik-ve-dinamik-yonlendirme
#@ degisken: süreler
k cargo build -q --release
k ./target/release/yonlendirme
# Üç fonksiyon yazdık, ikilide iki sembol var: &dyn ve Box<dyn> sürümleri aynı makine koduna derlendi ve birleştirildi
ks 'nm -C target/release/yonlendirme | grep toplam'
# Karşılaştırma için nightly'nin varsayılan derlemesi: yine iki sembol (fark araç zincirinden değil bayraktan)
ks 'cargo +nightly build -q --release --target-dir target/nightly && nm -C target/nightly/release/yonlendirme | grep toplam'
# Nedeni LLVM'in fonksiyon birleştirme (MergeFunctions) geçişi: nightly derleyicide bu geçiş kapatılınca üç sembol kalır
ks 'RUSTFLAGS="-Z merge-functions=disabled" cargo +nightly build -q --release --target-dir target/birlestirmesiz && nm -C target/birlestirmesiz/release/yonlendirme | grep toplam'
# Dinamik sürümde dolaylı çağrı: adres vtable'ın 3 numaralı girdisinden (+0x18) okunup çağrılıyor
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/yonlendirme | awk "/^0000.*<yonlendirme::toplam_kutulu>:/{f=1} f&&/^$/{exit} f" | grep -E "call|ret"'
# Statik sürümde hiç call yok: alan() satır içine alınmış, döngü açılmış (çarpma ve toplama komutları sayılıyor)
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/yonlendirme | awk "/^0000.*<yonlendirme::toplam_statik::<yonlendirme::Kare>>:/{f=1} f&&/^$/{exit} f" | grep -oE "\b(call|mulsd|addsd)\b" | sort | uniq -c'
