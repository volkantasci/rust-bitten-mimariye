#@ video: 048-monomorfizasyon
k cargo build -q --release
k ./target/release/monomorf
# Tek bir jenerik fonksiyon, ikilide üç ayrı makine kodu kopyası
ks 'nm -C target/release/monomorf | grep ikiye_katla'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/monomorf | grep -A3 "<monomorf::ikiye_katla::<"'
