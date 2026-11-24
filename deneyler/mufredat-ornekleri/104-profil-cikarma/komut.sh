#@ video: 104-profil-cikarma
#@ degisken: valgrind süreç numarası (==NNNN==) ve toplam komut sayısındaki küçük oynama (birkaç yüz komut); yüzdeler aynı kalır
k cargo build -q --release
k ./target/release/profil
# callgrind: her fonksiyonda yürütülen komut sayısı (zamanlamadan bağımsız, tekrarlanabilir)
ks 'valgrind --tool=callgrind --callgrind-out-file=/tmp/cg.out ./target/release/profil 2>&1 | grep -E "Collected|bulunan"'
# main'in çağırdıkları ve kapsayıcı (inclusive) maliyetleri; dosya yolları kısaltıldı
ks 'callgrind_annotate --inclusive=yes /tmp/cg.out 2>/dev/null | grep -E "PROGRAM TOTALS|=> .*profil::(yavas_arama|hizli_hazirlik|hizli_arama) " | sed -E "s#=> .*:(profil::[a-z_]+) #=> \1 #"'
# Kendi maliyeti (dahil olmayan görünüm): satır içine alınan kod çağıranın kendi maliyetinde görünür, sıralama ise çağrılan fonksiyonlarda
ks 'callgrind_annotate /tmp/cg.out 2>/dev/null | sed -n "/file:function/,+8p" | sed -E "s#/rustc/[0-9a-f]+/library/##; s#<u32, <u32 as core::cmp::PartialOrd>::lt>#<u32, …>#g; s# \[/work/[^]]*\]##"'
