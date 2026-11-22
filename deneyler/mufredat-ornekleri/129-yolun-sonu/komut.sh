#@ video: 129-yolun-sonu
#@ ortak: ortak-projeler/ambar
#@ ortak: ortak-projeler/kapi
#@ ortak: ortak-projeler/minik
cd ../ortak-projeler
for p in ambar kapi minik; do
  ks "cd $p && cargo test -q 2>&1 | grep 'test result' | head -n 1 | sed 's/^/$p: /'"
done
# Üç bitirme projesinin Rust satır sayısı
ks 'wc -l ambar/src/*.rs ambar/src/bin/*.rs kapi/src/*.rs kapi/src/bin/*.rs minik/src/*.rs minik/src/bin/*.rs | tail -n 1'
# Bu müfredat için konteynerde çalıştırılan deney sayısı
ks 'ls -d ../[0-9][0-9][0-9]-*/ | wc -l'
