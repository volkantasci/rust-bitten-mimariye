#@ video: 064-ic-degistirilebilirlik
#@ degisken: panik satırındaki iş parçacığı kimliği; Miri'nin ayırma numaraları sürüme bağlı
cd hucreler && k cargo run -q; cd ..
# Sızıntıyı bir araçla görelim: Miri program bitince iade edilmemiş öbek bloklarını raporlar
cd sizinti && k cargo +nightly miri run -q; cd ..
