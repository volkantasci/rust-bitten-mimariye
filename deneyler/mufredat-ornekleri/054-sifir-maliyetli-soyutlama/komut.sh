#@ video: 054-sifir-maliyetli-soyutlama
mkdir -p target
k rustc --edition 2024 -C opt-level=3 --crate-type=lib --emit=obj -o target/iki_yol.o iki_yol.rs
# Her fonksiyonun komut sayısı
ks 'objdump -d -M intel --no-show-raw-insn --disassemble=elle_dongu target/iki_yol.o | grep -cP "^\s+[0-9a-f]+:"'
ks 'objdump -d -M intel --no-show-raw-insn --disassemble=iterator_zinciri target/iki_yol.o | grep -cP "^\s+[0-9a-f]+:"'
# Vektör (SIMD) komutları: dört u32'yi aynı anda işleyen p... komutları
ks 'objdump -d -M intel --no-show-raw-insn --disassemble=iterator_zinciri target/iki_yol.o | grep -oE "\bp[a-z]+\b" | sort | uniq -c'
# Komut komut karşılaştırma (adresler ve etiketler ayıklanır)
ks 'diff <(objdump -d -M intel --no-show-raw-insn --disassemble=elle_dongu target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//") <(objdump -d -M intel --no-show-raw-insn --disassemble=iterator_zinciri target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//") && echo "makine kodu birebir aynı"'
# Sıra göz ardı edilirse: aynı komut kümesi mi?
ks 'diff <(objdump -d -M intel --no-show-raw-insn --disassemble=elle_dongu target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//" | sort) <(objdump -d -M intel --no-show-raw-insn --disassemble=iterator_zinciri target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//" | sort) && echo "aynı 64 komut, yalnızca sıralama farklı"'
