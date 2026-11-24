#@ video: 002-laboratuvar-kurulumu
#@ degisken: cargo'nun "Finished ... in X s" süresi; süreç kimlikleri (PID), konak adı (hostname) ve ad alanı kimlik numaraları
# Konteynerin içindeyiz: araç sürümleri ve çekirdek
k rustc --version
k cargo --version
k uname -r
ks 'grep PRETTY_NAME /etc/os-release'
# Yalıtım: bu sürecin ad alanları (namespace) ve konteynerin içinden görünen süreçler
k ls /proc/self/ns
k ps -e -o pid,comm
k hostname
# Yalıtımın kanıtı ad alanı kimliklerinde: ana makinede aynı komut başka numaralar verir
k readlink /proc/self/ns/pid /proc/self/ns/uts /proc/self/ns/net
# cgroup ad alanı: süreç kendi kontrol grubunu kök (/) olarak görür
k cat /proc/self/cgroup
# Kontrol grubunun bir kaynak sınırı: en fazla süreç sayısı
k cat /sys/fs/cgroup/pids.max
# Araç rafı: sürüm satırları
k cargo clippy --version
k rustfmt --version
k valgrind --version
ks 'strace -V | head -n 1'
ks 'gdb --version | head -n 1'
k hyperfine --version
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
