#@ video: 076-borular-ve-sinyaller
#@ degisken: iş parçacığı kimliği; sa_restorer adresi; ilk kapasite ölçümü (varsayılan 65 536 bayt; çalıştıran kullanıcının bütün borularının sayfa toplamı pipe-user-pages-soft sınırını aşmışsa 8192 bayt; rootless Podman'da bu toplama ana makinedeki süreçlerimiz de girer)
k cargo build -q
k ./target/debug/borular
# Rust çalışma zamanı, main'den önce SIGPIPE'ı "yok say" olarak ayarlar
ks 'strace -e trace=rt_sigaction ./target/debug/borular kapasite 2>&1 | grep SIGPIPE'
# Boru tamponunun kapasitesi: program önce çekirdeğe sorar (F_GETPIPE_SZ), sonra ölçer; okuma ucu kapanınca yazma
k ./target/debug/borular kapasite
# Yumuşak sınırı aşmış bir kullanıcı (pipe(7)). Kendi kullanıcımızın sayacını doldurmak ana makinedeki
# programlarımızı da etkilerdi; bu yüzden konteynerde başka bir kimlikle (uid 4242) çalışırız.
# uid_map: konteynerdeki 0 ana makinede bizim kullanıcımızdır, 1 ve sonrası ayrı bir kimlik aralığına eşlenir.
k cat /proc/self/uid_map
k setpriv --reuid=4242 --regid=4242 --clear-groups ./target/debug/borular sinirda
# Okuyan taraf boruyu erken kapatırsa: yazma EPIPE hatası döner, println! paniğe dönüşür
ks './target/debug/borular cok-yaz | head -n 2'
# Kendin dene: hatayı yakalayıp sessizce bitirmek
ks './target/debug/borular dikkatli-yaz | head -n 2; echo "yazanın çıkış kodu: ${PIPESTATUS[0]}"'
