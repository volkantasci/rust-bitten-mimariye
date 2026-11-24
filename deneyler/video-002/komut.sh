#@ video: 002-laboratuvar-kurulumu (ek deney: kanıtı ekrana taşıyan adımlar; ana deney mufredat-ornekleri/002-laboratuvar-kurulumu)
#@ degisken: strace satırındaki nodename (konak adı = konteyner kimliğinin ilk 12 karakteri); cargo'nun "Finished ... in X s" süresi
# 1) uname cevabını bir sistem çağrısıyla çekirdekten alır: release çekirdeğin sürümü, nodename konak adı
ks 'strace -v -e trace=uname uname -r'
# 2) İki paket, iki baskı: 2015 baskısındaki kütüphanede async bir fonksiyon adı; 2024 baskısındaki program onu çağırıyor
rm -rf calisma && mkdir -p calisma && cp -r eski yeni calisma/
cd calisma
k cat eski/Cargo.toml
k cat eski/src/lib.rs
k cat yeni/Cargo.toml
k cat yeni/src/main.rs
cd yeni
# Cargo her pakete kendi baskısını verir (rustc komut satırından yalnız ad ve baskı)
ks 'cargo build -v 2>&1 | grep -o -- "--crate-name [a-z]* --edition=[0-9]*"'
k cargo run
# 3) Kendin dene: kütüphanenin baskısını 2024 yapın (async artık anahtar sözcük)
cd ..
sed -i 's/edition = "2015"/edition = "2024"/' eski/Cargo.toml
k grep edition eski/Cargo.toml
cd yeni
k cargo build
