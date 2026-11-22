#@ video: 010-yigin-ve-obek
#@ degisken: adresler her çalıştırmada değişir (ASLR)
k cargo build -q
k ./target/debug/yigin_obek
