#@ video: 110-api-tasarimi
# non_exhaustive: dışarıdaki crate, joker (_) kol olmadan eşleştiremez
k cargo build -q -p istemci --bin istemci
# Joker kol eklenmiş sürüm: derlenir; must_use uyarısı atılan dönüş değerini yakalar
k cargo run -q -p istemci --bin duzgun
