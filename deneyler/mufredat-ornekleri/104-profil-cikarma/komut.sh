#@ video: 104-profil-cikarma
k cargo build -q --release
k ./target/release/profil
# callgrind: her fonksiyonda yürütülen komut sayısı (zamanlamadan bağımsız, tekrarlanabilir)
ks 'valgrind --tool=callgrind --callgrind-out-file=/tmp/cg.out ./target/release/profil 2>&1 | grep -E "Collected|bulunan"'
# main'in çağırdıkları ve kapsayıcı (inclusive) maliyetleri; dosya yolları kısaltıldı
ks 'callgrind_annotate --inclusive=yes /tmp/cg.out 2>/dev/null | grep -E "PROGRAM TOTALS|=> .*profil::(yavas_arama|hizli_hazirlik|hizli_arama) " | sed -E "s#=> .*:(profil::[a-z_]+) #=> \1 #"'
