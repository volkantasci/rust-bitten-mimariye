#@ video: 005-tam-sayilar
#@ degisken: panik satırındaki iş parçacığı kimliği (thread 'main' (N))
# Değer derleme anında biliniyorsa: derleyici reddeder
cd sabit-tasma && k cargo build -q; cd ..
# Değer çalışma anında: debug derlemede panik
k cargo run -q
# Release derlemede taşma denetimi kapalı: sessizce sarar
k cargo run -q --release
