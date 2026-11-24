#@ video: 072-veri-paralelligi
#@ degisken: süreler, adresler ve iş parçacığı başına düşen öğe sayıları
# Önbellek satırı boyutu (çekirdeğin bildirdiği, bayt)
k cat /sys/devices/system/cpu/cpu0/cache/index0/coherency_line_size
k cargo build -q --release
k ./target/release/parallel
# Kendin dene: iki sayaç arasında 48 bayt dolgu (aynı satır) ve 56 bayt dolgu (ayrı satır)
k ./target/release/parallel dolgu
