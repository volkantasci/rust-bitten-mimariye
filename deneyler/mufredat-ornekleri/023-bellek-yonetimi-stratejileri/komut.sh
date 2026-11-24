#@ video: 023-bellek-yonetimi-stratejileri
#@ degisken: miri çıktısındaki alloc numaraları; elle programının yazdığı adresler (ASLR)
#@ goster: memory leaked
# 1) Elle yönetim: unsafe std::alloc; iade etmeyi unutmak sessizdir, miri sızıntıyı bildirir
cd elle
k cargo run -q
k cargo +nightly miri run -q
cd ..
# 2) Referans sayımı (Rc): sayaç sıfıra inince hemen bırakma; çevrim ise hiç bırakılmaz
cd sayac
k cargo run -q
k cargo +nightly miri run -q
cd ..
# Kendin dene: çevrimin bir yönü Weak olunca ikisi de bırakılır
cd zayif && k cargo run -q; cd ..
# 3) İzleyen toplama (işaretle ve süpür), küçük bir benzetim: köklerden erişilemeyen çevrim de toplanır
cd isaretle-supur && k cargo run -q; cd ..
# 4) Sahiplik ve RAII: kapsam bitince deterministik bırakma, oluşturma sırasının tersiyle
cd raii && k cargo run -q; cd ..
