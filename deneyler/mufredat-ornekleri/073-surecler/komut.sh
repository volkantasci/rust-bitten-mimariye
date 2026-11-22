#@ video: 073-surecler
#@ degisken: süreç kimlikleri (pid)
k cargo build -q
k ./target/debug/surecler
# Çekirdek düzeyinde: yeni süreç (clone/clone3), yeni program (execve), bekleme (wait4)
ks 'strace -f -e trace=clone,clone3,vfork,execve,wait4,exit_group ./target/debug/surecler 2>&1 | grep -vE "^(ebeveyn|çocuk|uname|başlatıl)"'
