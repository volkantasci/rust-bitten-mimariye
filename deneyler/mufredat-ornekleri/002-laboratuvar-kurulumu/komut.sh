#@ video: 002-laboratuvar-kurulumu
#@ degisken: cargo'nun "Finished ... in X s" süresi
# Konteynerin içindeyiz: araç sürümleri ve çekirdek
k rustc --version
k cargo --version
k uname -r
ks 'grep PRETTY_NAME /etc/os-release'
# Sıfırdan bir proje: cargo new hangi edition'ı seçiyor?
rm -rf calisma && mkdir -p calisma && cd calisma
k cargo new merhaba
k cat merhaba/Cargo.toml
k cat merhaba/src/main.rs
cd merhaba
k cargo run
# Hangi edition'lar var? (geçersiz bir değer verip listeyi cargo'ya söyletiyoruz)
cd ..
k cargo new --edition 2027 deneme
ks 'rustc --help | grep -A3 -- "--edition"'
