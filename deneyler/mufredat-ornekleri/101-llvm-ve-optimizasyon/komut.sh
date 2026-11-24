#@ video: 101-llvm-ve-optimizasyon
#@ goster: .
mkdir -p target
k rustc --edition 2024 --crate-type=lib -C opt-level=0 -C debug-assertions=off -C overflow-checks=off --emit=llvm-ir -o target/O0.ll dongu.rs
k rustc --edition 2024 --crate-type=lib -C opt-level=3 --emit=llvm-ir -o target/O3.ll dongu.rs
ks 'echo "O0 satır: $(awk "/^define.*@kareler_toplami/,/^}/" target/O0.ll | wc -l)  O3 satır: $(awk "/^define.*@kareler_toplami/,/^}/" target/O3.ll | wc -l)"'
# O3: döngü (br/phi) kalmadı mı? Hangi komutlar var?
ks 'awk "/^define.*@kareler_toplami/,/^}/" target/O3.ll'
# Hangi geçişler bu fonksiyonu değiştirdi? (LLVM'in kendi raporu: -print-changed)
ks 'rustc --edition 2024 --crate-type=lib -C opt-level=3 --emit=llvm-ir -o target/O3b.ll -C llvm-args=-print-changed=quiet dongu.rs 2> target/gecisler.txt; grep -E "IR Dump After .*kareler_toplami" target/gecisler.txt | sed -E "s/^\*\*\* IR Dump After //; s/ \*\*\*$//"'
# Döngü geçişlerinden önce (LoopRotate sonrası): iki phi düğümlü döngü
ks 'awk "/IR Dump After LoopRotatePass/,/IR Dump After SimplifyCFGPass/" target/gecisler.txt | grep -v "IR Dump After SimplifyCFGPass"'
# IndVarSimplify sonrası: döngü gövdesi boşaldı (br i1 false), sonuç kapalı biçimle çıkışta hesaplanıyor
ks 'awk "/IR Dump After IndVarSimplifyPass/,/IR Dump After GVNPass/" target/gecisler.txt | grep -v "IR Dump After GVNPass"'
# Kendin dene: kareler yerine küpler (i * i * i); döngü yine kalkıyor mu?
ks 'sed "s/i \* i)/i * i * i)/" dongu.rs > target/kup.rs && rustc --edition 2024 --crate-type=lib -C opt-level=3 --emit=llvm-ir -o target/kup.ll target/kup.rs && awk "/^define.*@kareler_toplami/,/^}/" target/kup.ll | grep -E "^[a-z0-9.]+:|phi|ret"'
# Küp sürümünde ara hesap genişlikleri: 65 ve 67 bitlik tam sayılar
ks 'awk "/^define.*@kareler_toplami/,/^}/" target/kup.ll | grep -oE "i6[57]" | sort | uniq -c'
