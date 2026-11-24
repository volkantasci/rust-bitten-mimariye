#@ video: 048-tek-bicimlestirme
k cargo build -q --release
k ./target/release/tek_bicim
# Tek bir jenerik fonksiyon, ikilide üç ayrı makine kodu kopyası
ks 'nm -C target/release/tek_bicim | grep ikiye_katla'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/tek_bicim | grep -A3 "<tek_bicim::ikiye_katla::<"'
