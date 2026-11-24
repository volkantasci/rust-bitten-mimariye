#@ video: 070-kanallar-ve-mesajlasma
#@ degisken: milisaniye zaman damgaları ve aynı anlara düşen satırların sırası
k cargo build -q
# 1) Üç aşamalı boru hattı
k ./target/debug/kanallar boru-hatti
# 2) Kapasitesi 2 olan kanal: hızlı üretici, yavaş tüketici
k ./target/debug/kanallar 2
# Kendin dene: kapasite 0 (buluşma kanalı)
k ./target/debug/kanallar 0
# 3) Gönderilen değerin sahibi değişir
cd gonderince-tasinir && k cargo build -q; cd ..
