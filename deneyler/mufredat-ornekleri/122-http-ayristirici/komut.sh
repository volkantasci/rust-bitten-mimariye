#@ video: 122-http-ayristirici
#@ degisken: tarama ölçümündeki süreler ve oran
# Kontrol noktası: bu videonun sonundaki kod kapi/ dizininde.
cd kapi
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
k cargo build -q --release
# Bir istek dört parça hâlinde geliyor; boş satır ("\r\n\r\n") son iki parçaya bölünmüş
k ./target/release/kapi parca-parca 'GET /belgeler/not' '.txt HTTP/1.1\r\nHo' 'st: kapi\r\n\r' '\n'
# Gövdeli istek: başlıklar bitince durum Govde'ye geçer, Content-Length kadar bayt beklenir
k ./target/release/kapi parca-parca 'POST /kayit HTTP/1.1\r\nContent-Length: 5\r\n\r\nmer' 'ha'
# Bozuk istek satırı
k ./target/release/kapi parca-parca 'MERHABA\r\n\r\n'
# Taramanın maliyeti: 8 KiB'a yakın bir başlık bayt bayt geliyor
k ./target/release/kapi tarama-olc
