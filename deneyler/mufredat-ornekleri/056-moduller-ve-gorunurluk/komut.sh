#@ video: 056-moduller-ve-gorunurluk
cd kapsulleme && k cargo run -q; cd ..
# Gizli alana modülün dışından dokunmak derlenmez
cd gizli-alan && k cargo build -q; cd ..
# Alan pub olsaydı: program derlenir, değişmez bozulur ve ikili arama yanlış cevap verir
cd acik-alan && k cargo run -q; cd ..
