#@ video: 073-surecler
#@ degisken: süreç kimlikleri (pid), strace'teki adresler
k cargo build -q
k ./target/debug/surecler
# PATH: programın arandığı dizinler, bu sırayla
ks 'echo "$PATH"'
# C kütüphanesinin sürümü (posix_spawn'ı o uygular)
ks 'ldd --version | head -n 1'
# Çekirdek düzeyinde: yeni süreç (clone3), yeni program (execve), bekleme (wait4); strace'in kaydı programın çıktısına karışmasın diye dosyaya yazılır (-o), ek iletileri susturulur (-qq)
ks 'strace -f -qq -e trace=clone,clone3,vfork,execve,wait4,exit_group -o /tmp/iz.txt ./target/debug/surecler > /dev/null; cat /tmp/iz.txt'
# Zombi süreç: bitmiş ama çıkış durumu henüz alınmamış alt süreç
k ./target/debug/surecler zombi
# Kendin dene: PATH yalnız /usr/bin olunca execve ilk denemede bulur
ks 'PATH=/usr/bin strace -f -qq -e trace=execve -o /tmp/iz2.txt ./target/debug/surecler > /dev/null; grep -E "execve\(\"/usr/bin/(sh|uname)\"" /tmp/iz2.txt'
# Kendin dene: wait() hiç çağrılmazsa
k ./target/debug/surecler zombi-beklemesiz
