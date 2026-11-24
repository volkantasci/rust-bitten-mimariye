#@ video: 105-dallanma-tahmini
#@ degisken: süreler
k cargo build -q --release
k ./target/release/dallanma
# Dallanan sürüm: döngünün içinde koşullu atlama (jns: işaret biti 0 ise atla → x < 128)
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/dallanma | awk "/<dallanma::dallanan>:/,/^$/" | grep -E "\bj(ns|s)\b"'
# Dallanmasız sürüm: vektör karşılaştırma (pcmpgtb) + maske (pand); döngüde veriye bağlı atlama yok
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/dallanma | awk "/<dallanma::dallanmasiz>:/,/^$/" | grep -oE "\b(pcmpgtb|pand|paddq)\b" | sort | uniq -c'
